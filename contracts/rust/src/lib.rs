#![deny(warnings)]

///
/// Plantary NFT Smart Contract
/// adapted from https://github.com/near-examples/NFT by mykle
///
/// Implements blockchain ledger for plants and their fruit
///

use near_sdk::{env, near_bindgen, AccountId, Balance};
use near_sdk::collections::UnorderedMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand_seeder::{Seeder};

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc<'_> = near_sdk::wee_alloc::WeeAlloc::INIT;

mod token_bank;
use token_bank::{NEP4, TokenBank, TokenSet, TokenId};

mod constants;
use constants::{VeggieType, VeggieSubType, vtypes, P_POOL, H_POOL, P_PRICES, H_PRICES};

///
/// the veggie section
/// veggie is like a superclass of both plant and harvest.
/// (not necessarily the right way to do this in rust, i'm still learning ...)
///

#[derive(PartialEq, Clone, Debug, Serialize, BorshDeserialize, BorshSerialize)]
pub struct Veggie {
    pub vid: TokenId,
    pub vtype: VeggieType,
    pub vsubtype: VeggieSubType,
    pub parent: TokenId,
    pub dna: u64,
    pub meta_url: String,
}

impl Veggie {
    pub fn new(vid: TokenId, parent_vid: TokenId, vtype: VeggieType, vsubtype:VeggieSubType, dna: u64, meta_url: &String) -> Self {

        Self {
            vid: vid,
            vtype: vtype,           // plant or harvest 
            vsubtype: vsubtype,
            parent: parent_vid,
            dna: dna,
            meta_url: meta_url.to_string(),
            // rarity ...
        }
    }
}

// this is the external, JSON-compatible version for method calls.  (u64s are strings.)

pub type TokenJSON = String;

#[derive(PartialEq, Clone, Debug, Serialize, BorshDeserialize, BorshSerialize)]
pub struct VeggieJSON {
    pub vid: TokenJSON,
    pub vtype: VeggieType,
    pub vsubtype: VeggieSubType,
    pub parent: TokenJSON,
    pub dna: String,
    pub meta_url: String,
}

impl From<Veggie> for VeggieJSON {
    fn from(v: Veggie) -> Self {
        Self {
            vid: v.vid.to_string(),
            vtype: v.vtype,
            vsubtype: v.vsubtype,
            parent: v.parent.to_string(),
            dna: v.dna.to_string(),
            meta_url: v.meta_url
        }
    }
}

// I thought Rust would give me this for free ...
impl From<VeggieJSON> for Veggie {
    fn from(v: VeggieJSON) -> Self {
        Self {
            vid: v.vid.parse::<TokenId>().unwrap(),
            vtype: v.vtype,
            vsubtype: v.vsubtype,
            parent: v.parent.parse::<TokenId>().unwrap(),
            dna: v.dna.parse::<u64>().unwrap(),
            meta_url: v.meta_url,
        }
    }
}

pub trait Veggies {
    fn get_veggie_json(&self, vid_json: TokenJSON) -> VeggieJSON;
    fn count_owner_veggies(&self, owner_id: AccountId, vtype: VeggieType) -> u64;
    fn get_owner_veggies_page_json(&self, owner_id: AccountId, vtype: VeggieType, page_size: u16, page: u16) -> Vec<VeggieJSON>;

    fn mint_plant_json(&mut self, 
                    vsubtype: VeggieSubType,
                    )->VeggieJSON;

    fn delete_veggie_json(&mut self, vid_json: TokenJSON);

    fn harvest_plant_json(&mut self, parent_id: TokenJSON) -> VeggieJSON;
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

    fn get_veggie_json(&self, vid: TokenJSON) -> VeggieJSON {
        self.get_veggie(vid.parse::<TokenId>().unwrap()).into()
    }

    fn delete_veggie_json(&mut self, vid: TokenJSON){
        self.delete_veggie(vid.parse::<TokenId>().unwrap()).into()
    }

    #[payable]
    fn harvest_plant_json(&mut self, parent_id_json: TokenJSON) -> VeggieJSON {
        // confirm that we were paid the right amount:
        let parent_id = parent_id_json.parse::<TokenId>().unwrap();
        let parent = self.get_veggie(parent_id);
        self.paid_up(H_PRICES[parent.vsubtype as usize]);

        self.harvest_plant(parent_id).into()
    }

    fn get_owner_veggies_page_json(&self, owner_id: AccountId, vtype: VeggieType, page_size: u16, page: u16) -> Vec<VeggieJSON> {
        self.get_owner_veggies_page(owner_id, vtype, page_size, page).iter().map(|v| VeggieJSON::from(v.clone())).collect()
    }

    #[payable]
    fn mint_plant_json(&mut self, vsubtype: VeggieSubType) -> VeggieJSON {
        // TODO: only putting this here for now because I haven't figured out how to unit test payments properly ...
        // confirm that we were paid the right amount
        self.paid_up(P_PRICES[vsubtype as usize]);
        self.mint_plant(vsubtype).into()
    }

}

////////////////////////
// private methods used by Veggies
//
impl PlantaryContract {
    fn get_veggie(&self, vid: TokenId) -> Veggie {
        // TODO: check perms?
        let veggie = match self.veggies.get(&vid) {
            Some(c) => {
                c
            },
            None => {
                env::panic(b"Veggie does not exist.") 
            }
        };
        return veggie.clone();
    }

    fn delete_veggie(&mut self, vid: TokenId) {
        // panic if we're not the contract owner!
        self.only_owner();

        // delete from global list
        self.veggies.remove(&vid);
        // remove from ownership (should use burn_token)
        self.token_bank.token_to_account.remove(&vid);
    }

    fn mint_plant(&mut self,
                    vsubtype: VeggieSubType,
                    ) -> Veggie {
        // plants have no parents
        let parent_vid = 0;

        return self.create_veggie(vtypes::PLANT, vsubtype, parent_vid);
    }

    // harvest_plant() here, a plant veggie gives birth to a harvest veggie
    // (harvest in this case is a verb.)
    fn harvest_plant(&mut self, parent_id: TokenId) -> Veggie {
        // Assert: user owns this plant
        // Assert: this type of plant can even have a harvest
        // Assert: correct money was paid
        
        let parent = self.get_veggie(parent_id);

        // Assert: parent is a plant
        if parent.vtype != vtypes::PLANT {
            env::panic(b"non-plant harvest");
        }
        // for now, the harvest subtype is the same subtype as the parent plant
        let h = self.create_veggie(vtypes::HARVEST, parent.vsubtype, parent.vid);
        return h;
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

    // panic if invalid veggie types are attempted.
    fn check_vtype(&self, vtype: VeggieType){
        if ! (vtype == 0 || vtype == vtypes::PLANT || vtype == vtypes::HARVEST) {
            panic!("Unknown veggie type {}.", vtype);
        }
    }

    // panic if non-root tries to do a root thing
    fn only_owner(&mut self) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only contract owner can call this method.");
    }

    // panic unless exactly 'tokens' N are  attached
    fn paid_up(&self, tokens: Balance) {
        let yocto = tokens * 10u128.pow(24);
        let dep = env::attached_deposit();
        if dep != yocto {
            panic!("needed {} yn, received {}", yocto, dep);
        }
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
        let subtypes;
        if vtype == vtypes::PLANT {
            subtypes = &P_POOL[&vsubtype];
        } else {
            subtypes = &H_POOL[&vsubtype];
        }
        meta_url = subtypes[rng.gen_range(0, subtypes.len())].to_string();

        let dna: u64 = rng.gen();

        let v = Veggie::new(vid, parent_vid, vtype, vsubtype, dna, &meta_url);
        assert_eq!(vid, v.vid, "vid mismatch!");

        // record in the static list of veggies
        self.veggies.insert(&vid, &v); // vid has Copy trait; v does not.
        // record ownership in the nft structure
        self.token_bank.mint_token(env::predecessor_account_id(), vid);

        v
    }
}

// Our main contract object is PlantaryContract

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

    // debug
    pub fn get_veggie_keys(&self) -> Vec<String> {
        //self.veggies.keys().cloned().collect()
        self.veggies.keys().map(|i| i.to_string()).collect()
    }

}

// Expose NEP-4 interface of TokenBank
impl NEP4 for PlantaryContract {
    fn grant_access(&mut self, escrow_account_id: AccountId) {
        self.token_bank.grant_access(escrow_account_id)
    }

    fn revoke_access(&mut self, escrow_account_id: AccountId) {
        self.token_bank.revoke_access(escrow_account_id)
    }

    fn transfer_from(&mut self, owner_id: AccountId, new_owner_id: AccountId, token_id: TokenId) {
        self.token_bank.transfer_from(owner_id, new_owner_id, token_id)
    }

    fn transfer(&mut self, new_owner_id: AccountId, token_id: TokenId) {
        self.token_bank.transfer(new_owner_id, token_id) 
    }

    fn check_access(&self, account_id: &AccountId) -> bool {
        self.token_bank.check_access(account_id)
    }

    fn get_token_owner(&self, token_id: TokenId) -> String {
        self.token_bank.get_token_owner(token_id)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext, Balance};
    use constants::{vtypes, ptypes};

    fn to_ynear(near: Balance) -> Balance {
        near * 10u128.pow(24)
    }

    fn joe() -> AccountId {
        "joe.testnet".to_string()
    }
    fn robert() -> AccountId {
        "robert.testnet".to_string()
    }
    fn mike() -> AccountId {
        "mike.testnet".to_string()
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
            account_balance: 10u128.pow(28),
            account_locked_balance: 0,
            storage_usage,
            attached_deposit: 10u128.pow(27),
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
        let vid = v.vid;
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
        let mut c = get_context(robert(), 0);
        c.attached_deposit = to_ynear(P_PRICES[ptypes::PORTRAIT as usize]);
        testing_env!(c);
        let mut contract = PlantaryContract::new(robert());

            // create
        let p = contract.mint_plant(ptypes::PORTRAIT);
        let h = contract.harvest_plant(p.vid);
            // inspect
        assert_eq!(p.vid, h.parent, "parentage suspect");
        assert_eq!(p.vsubtype, h.vsubtype, "mismatched subtype");
    }

    // TODO: test that we can't harvest a plant we don't own.

    #[test]
    #[should_panic(
        expected = r#"Veggie does not exist."#
    )]
    fn create_get_delete_veggie_json(){
        testing_env!(get_context(robert(), 0));
        let mut contract = PlantaryContract::new(robert());
            // create
        let v = contract.create_veggie(vtypes::PLANT, ptypes::MONEY, 0);
            // inspect?
        assert_eq!(v.vtype, vtypes::PLANT, "vtype not saved");
        assert_eq!(v.vsubtype, ptypes::MONEY, "vsubtype not found.");
            // find?
        let vid_json = v.vid.to_string();
            // confirm
        let _foundv: Veggie = contract.get_veggie_json(vid_json.clone()).into(); // should not panic
        assert_eq!(v, _foundv, "veggie did not fetch right");
            // delete
        contract.delete_veggie_json(vid_json.clone()); 
            // confirm deleted
        let _nov = contract.get_veggie_json(vid_json); // should panic
    }

    #[test]
    fn count_owner_veggies(){
        let c = get_context(robert(), 0);
        testing_env!(c);
        let mut contract = PlantaryContract::new(robert());

        // mint some plants
        let _p1 = contract.mint_plant(ptypes::MONEY); 

        let _p2 = contract.mint_plant(ptypes::ORACLE);

        let _p3 = contract.mint_plant(ptypes::PORTRAIT);

        // harvest some fruit
        let _h1 = contract.harvest_plant(_p2.vid);
        let _h2 = contract.harvest_plant(_p3.vid);

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
        expected = r#"Unknown veggie type 23."#
    )]
    fn count_owner_veggies_unknown(){
        testing_env!(get_context(robert(), 0));
        let contract = PlantaryContract::new(robert());
        // count_owner_veggies() should panic for any unknown types
        assert_eq!(0, contract.count_owner_veggies(robert(), 23));
    }

    #[test]
    fn get_owner_veggies_page_1(){
        let c = get_context(robert(), 0);
        testing_env!(c);
        let mut contract = PlantaryContract::new(robert());

        // mint 23  plants
        for _n in 0..22 {
            contract.mint_plant(ptypes::MONEY);
        }
        let _p23 = contract.mint_plant(ptypes::ORACLE);

        // mint 13 harvests
        for _o in 0..13 {
            contract.harvest_plant(_p23.vid);
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

    }

    #[test]
    fn get_owner_veggies_page_2(){
        testing_env!(get_context(robert(), 0));
        let mut contract = PlantaryContract::new(robert());

        // mint 23  plants
        for _n in 0..22 {
            contract.mint_plant(ptypes::MONEY);
        }
        let _p23 = contract.mint_plant(ptypes::ORACLE);

        // mint 13 harvests
        for _o in 0..13 {
            contract.harvest_plant(_p23.vid);
        }

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
        expected = r#"Unknown veggie type 23."#
    )]
    fn get_owner_veggies_unknown(){
        testing_env!(get_context(robert(), 0));
        let contract = PlantaryContract::new(robert());
        // count_owner_veggies() should panic for any unknown types
        let _foo = contract.get_owner_veggies_page(robert(), 23, 1, 1); // panic!
    }

    // From here down I've just duplicated the unit tests in TokenBank.rs ,
    // to test our wrapper methods around that object.

        #[test]
        fn grant_access() {
            let context = get_context(robert(), 0);
            testing_env!(context);
            let mut tb = TokenBank::new();
            let length_before = tb.account_gives_access.len();
            assert_eq!(0, length_before, "Expected empty account access Map.");
            tb.grant_access(mike());
            tb.grant_access(joe());
            let length_after = tb.account_gives_access.len();
            assert_eq!(1, length_after, "Expected an entry in the account's access Map.");
            let predecessor_hash = env::sha256(robert().as_bytes());
            let num_grantees = tb.account_gives_access.get(&predecessor_hash).unwrap();
            assert_eq!(2, num_grantees.len(), "Expected two accounts to have access to predecessor.");
        }

        #[test]
        #[should_panic(
            expected = r#"Access does not exist."#
        )]
        fn revoke_access_and_panic() {
            let context = get_context(robert(), 0);
            testing_env!(context);
            let mut tb = TokenBank::new();
            tb.revoke_access(joe());
        }

        #[test]
        fn add_revoke_access_and_check() {
            // Joe grants access to Robert
            let mut context = get_context(joe(), 0);
            testing_env!(context);
            let mut tb = TokenBank::new();
            tb.grant_access(robert());

            // does Robert have access to Joe's account? Yes.
            context = get_context(robert(), env::storage_usage());
            testing_env!(context);
            let mut robert_has_access = tb.check_access(&joe());
            assert_eq!(true, robert_has_access, "After granting access, check_access call failed.");

            // Joe revokes access from Robert
            context = get_context(joe(), env::storage_usage());
            testing_env!(context);
            tb.revoke_access(robert());

            // does Robert have access to Joe's account? No
            context = get_context(robert(), env::storage_usage());
            testing_env!(context);
            robert_has_access = tb.check_access(&joe());
            assert_eq!(false, robert_has_access, "After revoking access, check_access call failed.");
        }

        #[test]
        fn mint_token_get_token_owner() {
            let context = get_context(robert(), 0);
            testing_env!(context);
            let mut tb = TokenBank::new();
            tb.mint_token(mike(), 19u64);
            let owner = tb.get_token_owner(19u64);
            assert_eq!(mike(), owner, "Unexpected token owner.");
        }

        #[test]
        #[should_panic(
            expected = r#"Attempt to transfer a token with no access."#
        )]
        fn transfer_from_with_no_access_should_fail() {
            // Mike owns the token.
            // Robert is trying to transfer it to Robert's account without having access.
            let context = get_context(robert(), 0);
            testing_env!(context);
            let mut tb = TokenBank::new();
            let token_id = 19u64;
            tb.mint_token(mike(), token_id);
            tb.transfer_from(mike(), robert(), token_id.clone());
        }

        #[test]
        fn transfer_from_with_escrow_access() {
            // Escrow account: robert.testnet
            // Owner account: mike.testnet
            // New owner account: joe.testnet
            let mut context = get_context(mike(), 0);
            testing_env!(context);
            let mut tb = TokenBank::new();
            let token_id = 19u64;
            tb.mint_token(mike(), token_id);
            // Mike grants access to Robert
            tb.grant_access(robert());

            // Robert transfers the token to Joe
            context = get_context(robert(), env::storage_usage());
            testing_env!(context);
            tb.transfer_from(mike(), joe(), token_id.clone());

            // Check new owner
            let owner = tb.get_token_owner(token_id.clone());
            assert_eq!(joe(), owner, "Token was not transferred after transfer call with escrow.");
        }

        #[test]
        #[should_panic(
            expected = r#"Attempt to transfer a token from wrong owner."#
        )]
        fn transfer_from_with_escrow_access_wrong_owner_id() {
            // Escrow account: robert.testnet
            // Owner account: mike.testnet
            // New owner account: joe.testnet
            let mut context = get_context(mike(), 0);
            testing_env!(context);
            let mut tb = TokenBank::new();
            let token_id = 19u64;
            tb.mint_token(mike(), token_id);
            // Mike grants access to Robert
            tb.grant_access(robert());

            // Robert transfers the token to Joe
            context = get_context(robert(), env::storage_usage());
            testing_env!(context);
            tb.transfer_from(robert(), joe(), token_id.clone());
        }

        #[test]
        fn transfer_from_with_your_own_token() {
            // Owner account: robert.testnet
            // New owner account: joe.testnet

            testing_env!(get_context(robert(), 0));
            let mut tb = TokenBank::new();
            let token_id = 19u64;
            tb.mint_token(robert(), token_id);

            // Robert transfers the token to Joe
            tb.transfer_from(robert(), joe(), token_id.clone());

            // Check new owner
            let owner = tb.get_token_owner(token_id.clone());
            assert_eq!(joe(), owner, "Token was not transferred after transfer call with escrow.");
        }

        #[test]
        #[should_panic(
            expected = r#"Attempt to call transfer on tokens belonging to another account."#
        )]
        fn transfer_with_escrow_access_fails() {
            // Escrow account: robert.testnet
            // Owner account: mike.testnet
            // New owner account: joe.testnet
            let mut context = get_context(mike(), 0);
            testing_env!(context);
            let mut tb = TokenBank::new();
            let token_id = 19u64;
            tb.mint_token(mike(), token_id);
            // Mike grants access to Robert
            tb.grant_access(robert());

            // Robert transfers the token to Joe
            context = get_context(robert(), env::storage_usage());
            testing_env!(context);
            tb.transfer(joe(), token_id.clone());
        }

        #[test]
        fn transfer_with_your_own_token() {
            // Owner account: robert.testnet
            // New owner account: joe.testnet

            testing_env!(get_context(robert(), 0));
            let mut tb = TokenBank::new();
            let token_id = 19u64;
            tb.mint_token(robert(), token_id);

            // Robert transfers the token to Joe
            tb.transfer(joe(), token_id.clone());

            // Check new owner
            let owner = tb.get_token_owner(token_id.clone());
            assert_eq!(joe(), owner, "Token was not transferred after transfer call with escrow.");
        }
}

