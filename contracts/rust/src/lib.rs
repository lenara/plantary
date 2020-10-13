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
use token_bank::{TokenBank, TokenId};

mod constants;
use constants::{VeggieType, VeggieSubType, vtypes, htypes, P_POOL};

///
/// the veggie section
/// veggie is like a superclass of both plant and harvest.
/// (not necessarily the right way to do this in rust, i'm still learning ...)
///

#[derive(PartialEq, Debug, Serialize, BorshDeserialize, BorshSerialize)]
pub struct Veggie {
    pub id: TokenId,
    pub vtype: VeggieType,
    pub vsubtype: VeggieSubType,
    pub parent: TokenId,
    pub dna: u64,
    pub meta_url: String,
}

impl Veggie {
    //pub fn new(rng: &mut ChaCha8Rng, vtype: VeggieType, vsubtype:VeggieSubType) -> Self {
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
    // TODO: private
    fn create_veggie(&mut self, 
                    vtype: VeggieType,
                    vsubtype: VeggieSubType,
                    parent_vid: TokenId,
                    ) -> Veggie;

    fn delete_veggie(&mut self, vid: TokenId);

    fn mint_plant(&mut self, 
                    vsubtype: VeggieSubType,
                    )->Veggie;


    // TODO: deprecate!
    fn get_plant(&self, vid: TokenId) -> Veggie;
    fn get_owner_plants(&self, owner_id: AccountId) -> Vec<Veggie>;

    // TODO: better:
    fn get_veggie(&self, vid: TokenId) -> Veggie;
    fn count_owner_veggies(&self, owner_id: AccountId, vtype: u8) -> u64;
    //fn get_owner_veggies_page(&self, owner_id: AccountId, vtype: u8, page_size: u16, page: u16) -> Vec<Veggie>;

    //fn get_plants(&self, owner_id: AccountId) -> Vec<Veggie>;

    fn delete_plant(&mut self, vid: TokenId);

}

// veggie implementation
//

#[near_bindgen]
impl Veggies for PlantaryContract {
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

    fn count_owner_veggies(&self, owner_id: AccountId, vtype: u8) -> u64 {
        let tokens = self.token_bank.get_owner_tokens(&owner_id);

            // type 0 means "count all veggies"
        if vtype == 0  { 
            return tokens.len();
        }
        
        if ! (vtype == vtypes::PLANT || vtype == vtypes::HARVEST) {
            env::panic(b"Unknown veggie type.") 
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
        // TODO: check perms?
        // delete from global list
        self.veggies.remove(&vid);
        // remove from ownership (should use burn_token)
        self.token_bank.token_to_account.remove(&vid);
    }

    // same thing for plants

    #[payable]
    fn mint_plant(&mut self,
                    vsubtype: VeggieSubType,
                    ) -> Veggie {
        // plants have no parents
        let parent_vid = 0;

        // TODO: confirm that we were paid the right amount!
        return self.create_veggie(vtypes::PLANT, vsubtype, parent_vid);
    }

    fn get_plant(&self, vid: TokenId) -> Veggie {
        return self.get_veggie(vid);
    }

    fn get_owner_plants(&self, owner_id: AccountId) -> Vec<Veggie> {
        // get all owner tokens:
        let owner_tokens = self.token_bank.get_owner_tokens(&owner_id).to_vec();
        // look up their veggies
        let mut owner_plants: Vec<Veggie> = Vec::new();
        for ot in owner_tokens {
            let ov = self.get_veggie(ot);
            if ov.vtype == vtypes::PLANT { owner_plants.push(ov); }
        }

        owner_plants
    }

    fn delete_plant(&mut self, vid: TokenId){
        return self.delete_veggie(vid);
    }
}

/// end Plant section

/// the Harvest section, which also delegates everything to Veggie
///

pub trait Harvests {
    fn create_harvest(&mut self,
                    vsubtype: VeggieSubType,
                    parent_vid: TokenId,
                    )->Veggie;

    fn get_harvest(self, vid: TokenId) -> Veggie;

    fn delete_harvest(&mut self, vid: TokenId);

    fn harvest_plant(&mut self, parent: &Veggie) -> Veggie;
}

impl Harvests for PlantaryContract {
    fn create_harvest(&mut self,
                    vsubtype: VeggieSubType,
                    parent_vid: TokenId,
                    ) -> Veggie {
        return self.create_veggie(vtypes::HARVEST, vsubtype, parent_vid);
    }

    fn get_harvest(self, vid: TokenId) -> Veggie {
        return self.get_veggie(vid);
    }

    fn delete_harvest(&mut self, vid: TokenId){
        return self.delete_veggie(vid);
    }

    // harvest_plant() here, a plant veggie gives birth to a harvest veggie
    // (harvest in this case is a verb.)
    fn harvest_plant(&mut self, parent: &Veggie) -> Veggie {
        // Assert: parent is a plant
        if parent.vtype != vtypes::PLANT {
            env::panic(b"non-plant harvest");
        }
        // TODO: for every plant type there is a set of allowed harvest types, or none allowed)
        let h = self.create_harvest(htypes::GENERIC, parent.id);
        return h;
    }
}

/// end Harvest section

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct PlantaryContract {
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

    /*
    /// helper function determining contract ownership
    /// Really these token functions all need some clearer security framework.
    fn only_owner(&mut self) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only contract owner can call this method.");
    }
    */
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
            assert_eq!(v.vsubtype, ptypes::MONEY, "subtype not found.");
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

        #[test]
        #[should_panic(
            expected = r#"Veggie does not exist."#
        )]
        fn create_delete_plant(){
            testing_env!(get_context(robert(), 0));
            let mut contract = PlantaryContract::new(robert());

                // create
            let p = contract.mint_plant(ptypes::MONEY);
                // inspect
            assert_eq!(p.vtype, vtypes::PLANT, "vtype not saved");
            assert_eq!(p.vsubtype, ptypes::MONEY, "vsubtype not saved");
                // find
            let same_p = contract.get_plant(p.id);
            assert_eq!(p.id, same_p.id, "cant get plant");
            assert_eq!(p.vtype, same_p.vtype, "cant get plant");
            assert_eq!(p.vsubtype, same_p.vsubtype, "cant get plant");
                // delete
            contract.delete_plant(p.id);
                // confirm deleted
            let _nop = contract.get_plant(p.id); // should panic
        }

        // TODO: Test that we are charged some NEAR tokens when we mint a plant

        #[test]
        fn harvest_plant(){
            testing_env!(get_context(robert(), 0));
            let mut contract = PlantaryContract::new(robert());

                // create
            let p = contract.mint_plant(ptypes::MONEY);
            let h = contract.harvest_plant(&p);
                // inspect
            assert_eq!(p.id, h.parent, "parentage suspect");
        }

        // TODO: test that we can't harvest a plant we don't own.

        #[test]
        fn get_owner_plants(){
            testing_env!(get_context(robert(), 0));
            let mut contract = PlantaryContract::new(robert());

            // mint some plants
            let _p1 = contract.mint_plant(ptypes::MONEY);
            let _p2 = contract.mint_plant(ptypes::ORACLE);
            let _p3 = contract.mint_plant(ptypes::PORTRAIT);
            // harvest some fruit
            let _h1 = contract.harvest_plant(&_p1);
            let _h2 = contract.harvest_plant(&_p1);
            let _h3 = contract.harvest_plant(&_p2);

            // TODO: mint some other plant as some other user than robert() ...

            // get_owner_tokens should have it all for robert()
            let ot = contract.token_bank.get_owner_tokens(&robert());
            assert_eq!(ot.len(), 6, "wrong number of veggies");

            // get_owner_plants should just have plants
            let op = contract.get_owner_plants(robert());
            assert_eq!(op.len(), 3, "wrong number of plants");
        }

        #[test]
        fn count_owner_veggies(){
            testing_env!(get_context(robert(), 0));
            let mut contract = PlantaryContract::new(robert());

            // mint some plants
            let _p1 = contract.mint_plant(ptypes::MONEY);
            let _p2 = contract.mint_plant(ptypes::ORACLE);
            let _p3 = contract.mint_plant(ptypes::PORTRAIT);
            // harvest some fruit
            let _h1 = contract.harvest_plant(&_p1);
            let _h2 = contract.harvest_plant(&_p1);

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
            // count_owner_veggies should panic for unknown types
            assert_eq!(0, contract.count_owner_veggies(robert(), 23));
        }
}

