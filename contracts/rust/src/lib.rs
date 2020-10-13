#![deny(warnings)]

///
/// Plantary NFT Smart Contract
/// adapted from https://github.com/near-examples/NFT by mykle
///
/// Implements blockchain ledger for plants and their fruit
///

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId};

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand_seeder::{Seeder};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod token_bank;
use token_bank::{TokenBank, TokenSet, TokenId};

mod constants;
use constants::{VeggieType, VeggieSubType, vtypes, htypes, P_POOL};

///
/// the veggie section
/// veggie is like a superclass of both plant and harvest.
/// (not necessarily the right way to do this in rust, i'm still learning ...)
///

#[derive(PartialEq, Clone, Debug, Serialize, BorshDeserialize, BorshSerialize)]
pub struct Veggie {
    pub id: TokenId,
    pub vtype: VeggieType,
    pub vsubtype: VeggieSubType,
    pub parent: TokenId,
    pub dna: u64,
    pub meta_url: String,
}

impl Veggie {
    pub fn new(id: TokenId, parent_vid: TokenId, vtype: VeggieType, vsubtype:VeggieSubType, dna: u64, meta_url: &String) -> Self {

        Self {
            id: id,
            vtype: vtype,           // plant or harvest 
            vsubtype: vsubtype,
            parent: parent_vid,
            dna: dna,
            meta_url: meta_url.to_string(),
            // rarity ...
        }
    }
}

pub trait Veggies {
    fn get_veggie(&self, vid: TokenId) -> Veggie;
    fn count_owner_veggies(&self, owner_id: AccountId, vtype: VeggieType) -> u64;
    fn get_owner_veggies_page(&self, owner_id: AccountId, vtype: VeggieType, page_size: u16, page: u16) -> Vec<Veggie>;

    fn mint_plant(&mut self, 
                    vsubtype: VeggieSubType,
                    )->Veggie;

    fn delete_veggie(&mut self, vid: TokenId);

    fn harvest_plant(&mut self, parent_id: TokenId) -> Veggie;
}

// public veggies implementation
//
#[near_bindgen]
impl Veggies for PlantaryContract {

    fn count_owner_veggies(&self, owner_id: AccountId, vtype: VeggieType) -> u64 {
        self.check_vtype(vtype);

        let tokens = self.token_bank.get_owner_tokens(&owner_id);
            // type 0 means "count all veggies"
        if vtype == 0  { 
            return tokens.len();
        }
        
        let mut count = 0;
        for t in tokens.iter() {
            if self.veggies.get(&t).unwrap().vtype == vtype {
                count += 1;
            }
        }
        
        count
    }

    fn get_veggie(&self, vid: TokenId) -> Veggie {
        // TODO: check perms?
        let veggie = match self.veggies.get(&vid) {
            Some(c) => {
                c
            },
            None => {
                env::panic(b"Veggie does not exist.") // TODO: find pattern for throwing exception
            }
        };
        return veggie;
    }

    fn delete_veggie(&mut self, vid: TokenId) {
        // panic if we're not the contract owner!
        self.only_owner();

        // delete from global list
        self.veggies.remove(&vid);
        // remove from ownership (should use burn_token)
        self.token_bank.token_to_account.remove(&vid);
    }

    #[payable]
    fn mint_plant(&mut self,
                    vsubtype: VeggieSubType,
                    ) -> Veggie {
        // plants have no parents
        let parent_vid = 0;

        // TODO: confirm that we were paid the right amount!
        return self.create_veggie(vtypes::PLANT, vsubtype, parent_vid);
    }

    fn get_owner_veggies_page(&self, owner_id: AccountId, vtype: VeggieType, page_size: u16, page: u16) -> Vec<Veggie> {
        self.check_vtype(vtype);
        // get all owner tokens
        let tokens:TokenSet = self.token_bank.get_owner_tokens(&owner_id); // TokenSet == UnorderedSet<TokenId>
        // convert to all owner plants
        let mut owner_veggies: Vec<Veggie> = Vec::new();
        for ot in tokens.iter() {
            let ov = self.get_veggie(ot);
            if (vtype == 0) || (vtype == ov.vtype) { owner_veggies.push(ov); }
        }

        // calculate page, return it
        let count = owner_veggies.len();

        // pagesize 0?  try to return all results 
        if page_size == 0 {
            return owner_veggies;
        }

        let startpoint: usize = page_size as usize * page as usize;
        if startpoint > count { return Vec::new(); }

        let mut endpoint : usize =  startpoint + page_size as usize;
        if endpoint > count { endpoint = count; }

        owner_veggies[startpoint .. endpoint].to_vec()
    }

    // harvest_plant() here, a plant veggie gives birth to a harvest veggie
    // (harvest in this case is a verb.)
    #[payable]
    fn harvest_plant(&mut self, parent_id: TokenId) -> Veggie {
        // Assert: user owns this plant
        // Assert: this type of plant can even have a harvest
        // Assert: correct money was paid
        
        let parent = self.get_veggie(parent_id);

        // Assert: parent is a plant
        if parent.vtype != vtypes::PLANT {
            env::panic(b"non-plant harvest");
        }
        // TODO: for every plant type there is a set of allowed harvest types, or none allowed)
        let h = self.create_veggie(vtypes::HARVEST, htypes::GENERIC, parent.id);
        return h;
    }
}

////////////////////////
// private methods used by Veggies
//
impl PlantaryContract {
    // panic if invalid veggie types are attempted.
    fn check_vtype(&self, vtype: VeggieType){
        if ! (vtype == 0 || vtype == vtypes::PLANT || vtype == vtypes::HARVEST) {
            env::panic(b"Unknown veggie type.") 
        }
    }

    // panic if non-root tries to do a root thing
    fn only_owner(&mut self) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only contract owner can call this method.");
    }

    // create a veggie with tokenID and random properties
    fn create_veggie(&mut self, 
                    vtype: VeggieType,
                    vsubtype: VeggieSubType,
                    parent_vid: TokenId,
                    ) -> Veggie {

        // seed RNG
        let mut rng: ChaCha8Rng = Seeder::from(env::random_seed()).make_rng();

        // generate veggie-unique id
        let mut vid: TokenId;
        loop { 
            vid = rng.gen();
            match self.veggies.get(&vid) {
                None => { break; }
                Some(_) => { continue; }
            }
        }

        // pick a meta URL at random from the plant pool for the given subtype
        let meta_url: String;
        if vtype == vtypes::PLANT {
            let subtypes = &P_POOL[&vsubtype];
            meta_url = subtypes[rng.gen_range(0, subtypes.len())].to_string();
        } else {
            // TODO
            meta_url = "TBD".to_string();
        }

        let dna: u64 = rng.gen();

        let v = Veggie::new(vid, parent_vid, vtype, vsubtype, dna, &meta_url);

        // record in the static list of veggies
        self.veggies.insert(&v.id, &v);
        // record ownership in the nft structure
        self.token_bank.mint_token(env::predecessor_account_id(), v.id);
        return v;
    }
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct PlantaryContract {
    // first international bank of NFTs
    pub token_bank: TokenBank,
    // owner of the contract:
    pub owner_id: AccountId,
    // metadata storage
    pub veggies: UnorderedMap<TokenId, Veggie>,
}

impl Default for PlantaryContract {
    fn default() -> Self {
        panic!("plantary should be initialized before usage")
    }
}

// Public contract methods, callable on interwebs:
#[near_bindgen]
impl PlantaryContract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Owner's account ID is invalid.");
        assert!(!env::state_exists(), "Already initialized");
        Self {
            token_bank: TokenBank::new(),
            owner_id,
            veggies: UnorderedMap::new(b"veggies".to_vec()),
        }
    }

    pub fn get_owner_tokens(&self, owner_id: &AccountId) -> Vec<TokenId> {
        self.token_bank.get_owner_tokens(&owner_id).to_vec()
    }

}

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    use constants::{vtypes, ptypes};

    fn robert() -> AccountId {
        "robert.testnet".to_string()
    }

    // part of writing unit tests is setting up a mock context
    // this is a useful list to peek at when wondering what's available in env::*
    fn get_context(predecessor_account_id: String, storage_usage: u64) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "jane.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
                predecessor_account_id,
                input: vec![],
                block_index: 0,
                block_timestamp: 0,
                account_balance: 10 ^ 28,
                account_locked_balance: 0,
                storage_usage,
                attached_deposit: 10 ^ 27,
                prepaid_gas: 10u64.pow(18),
                random_seed: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                is_view: false,
                output_data_receivers: vec![],
                epoch_height: 19,
            }
        }

        #[test]
        #[should_panic(
            expected = r#"Veggie does not exist."#
        )]
        fn create_delete_veggie() {
            testing_env!(get_context(robert(), 0));
            let mut contract = PlantaryContract::new(robert());
                // create
            let v = contract.create_veggie(vtypes::PLANT, ptypes::MONEY, 0);
                // inspect?
            assert_eq!(v.vtype, vtypes::PLANT, "vtype not saved");
            assert_eq!(v.vsubtype, ptypes::MONEY, "vsubtype not found.");
                // find?
            let vid = v.id;
                // confirm
            let _foundv = contract.get_veggie(vid); // should not panic
            assert_eq!(v, _foundv, "veggie did not fetch right");
                // delete
            contract.delete_veggie(vid); // TODO: should veggie have its own method? so like v.burn() ...
                // confirm deleted
            let _nov = contract.get_veggie(vid); // should panic
        }

        // TODO: test we can't delete a veggie we don't own (unless we are contract owner)


        // TODO: Test that we are charged some NEAR tokens when we mint a plant

        #[test]
        fn harvest_plant(){
            testing_env!(get_context(robert(), 0));
            let mut contract = PlantaryContract::new(robert());

                // create
            let p = contract.mint_plant(ptypes::MONEY);
            let h = contract.harvest_plant(p.id);
                // inspect
            assert_eq!(p.id, h.parent, "parentage suspect");
        }

        // TODO: test that we can't harvest a plant we don't own.


        #[test]
        fn count_owner_veggies(){
            testing_env!(get_context(robert(), 0));
            let mut contract = PlantaryContract::new(robert());

            // mint some plants
            let _p1 = contract.mint_plant(ptypes::MONEY);
            let _p2 = contract.mint_plant(ptypes::ORACLE);
            let _p3 = contract.mint_plant(ptypes::PORTRAIT);
            // harvest some fruit
            let _h1 = contract.harvest_plant(_p1.id);
            let _h2 = contract.harvest_plant(_p1.id);

            // count_owner_veggies should return 5 for type 0, which is "all"
            assert_eq!(5, contract.count_owner_veggies(robert(), 0));
            // count_owner_veggies should return 3 for type PLANT
            assert_eq!(3, contract.count_owner_veggies(robert(), vtypes::PLANT));
            // count_owner_veggies should return 2 for type HARVEST
            assert_eq!(2, contract.count_owner_veggies(robert(), vtypes::HARVEST));
            // this person has no veggies
            assert_eq!(0, contract.count_owner_veggies("jane.testnet".to_string(), 0));
        }

        #[test]
        #[should_panic(
            expected = r#"Unknown veggie type."#
        )]
        fn count_owner_veggies_unknown(){
            testing_env!(get_context(robert(), 0));
            let contract = PlantaryContract::new(robert());
            // count_owner_veggies() should panic for any unknown types
            assert_eq!(0, contract.count_owner_veggies(robert(), 23));
        }

        #[test]
        fn get_owner_veggies_page(){
            testing_env!(get_context(robert(), 0));
            let mut contract = PlantaryContract::new(robert());

            // mint 23  plants
            for _n in 0..22 {
                contract.mint_plant(ptypes::MONEY);
            }
            let _p23 = contract.mint_plant(ptypes::ORACLE);

            // mint 13 harvests
            for _o in 0..13 {
                contract.harvest_plant(_p23.id);
            }

            // test plants:
            // get three pages of size 7
            // check that they are all full
            for p in 0..3 {
                let tokens = contract.get_owner_veggies_page(robert(), vtypes::PLANT, 7,p);
                assert_eq!(tokens.len(), 7, "bad plant page size");
            }

            // get another page of size 7
            // check that it is only 2 items long
            let tokens = contract.get_owner_veggies_page(robert(), vtypes::PLANT, 7,3);
            assert_eq!(tokens.len(), 2, "bad plant end page size");

            // get yet another page, should be empty.
            let tokens = contract.get_owner_veggies_page(robert(), vtypes::PLANT, 7,100);
            assert_eq!(tokens.len(), 0, "bad plant blank page size");

            // check that we can get the whole thing in one big slurp
            let tokens = contract.get_owner_veggies_page(robert(), vtypes::PLANT, 23,0);
            assert_eq!(tokens.len(), 23, "bad plant total page size");

            let tokens = contract.get_owner_veggies_page(robert(), vtypes::PLANT, 0,0);
            assert_eq!(tokens.len(), 23, "bad plant total page size");

            let tokens = contract.get_owner_veggies_page(robert(), vtypes::PLANT, 100,0);
            assert_eq!(tokens.len(), 23, "bad plant total page size");


            // test harvests:
            for p in 0..2 {
                let tokens = contract.get_owner_veggies_page(robert(), vtypes::HARVEST, 4,p);
                assert_eq!(tokens.len(), 4, "bad harvest page size");
            }

            let tokens = contract.get_owner_veggies_page(robert(), vtypes::HARVEST, 7,100);
            assert_eq!(tokens.len(), 0, "bad harvest blank page size");

            let tokens = contract.get_owner_veggies_page(robert(), vtypes::HARVEST, 13,0);
            assert_eq!(tokens.len(), 13, "bad harvest total page size");

            let tokens = contract.get_owner_veggies_page(robert(), vtypes::HARVEST, 0,0);
            assert_eq!(tokens.len(), 13, "bad harvest total page size");

            let tokens = contract.get_owner_veggies_page(robert(), vtypes::HARVEST, 100,0);
            assert_eq!(tokens.len(), 13, "bad harvest total page size");

            let tokens = contract.get_owner_veggies_page(robert(), vtypes::HARVEST, 6,2);
            assert_eq!(tokens.len(), 1, "bad harvest end page size");

        }

        #[test]
        #[should_panic(
            expected = r#"Unknown veggie type."#
        )]
        fn get_owner_veggies_unknown(){
            testing_env!(get_context(robert(), 0));
            let contract = PlantaryContract::new(robert());
            // count_owner_veggies() should panic for any unknown types
            contract.get_owner_veggies_page(robert(), 23, 1, 1); // panic!
        }
}

