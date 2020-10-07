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
//use near_sdk::collections::UnorderedSet;
use near_sdk::{env, near_bindgen, AccountId};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod token_bank;
use token_bank::{TokenBank, TokenId};

mod constants;
use constants::{VeggieType, VeggieSubType, vtypes, htypes};

///
/// the veggie section
/// veggie is like a superclass of both plant and harvest.
/// (not necessarily the right way to do this in rust, i'm still learning ...)
///

#[derive(Serialize, BorshDeserialize, BorshSerialize)]
pub struct Veggie {
    pub id: TokenId,
    pub vtype: VeggieType,
    pub vsubtype: VeggieSubType,
    pub parent: TokenId,
}

impl Veggie {
    pub fn new(id: TokenId, vtype: VeggieType, vsubtype:VeggieSubType) -> Self {
        Self {
            id: id,
            vtype: vtype,           // plant or harvest 
            vsubtype: vsubtype,
            parent: 0,
            // dna ...
            // rarity ...
        }
    }
}

// veggie traits
//


pub trait Veggies {
    fn create_veggie(&mut self, 
                    vtype: VeggieType,
                    vsubtype: VeggieSubType,
                    ) -> Veggie;

    fn get_veggie(&self, vid: TokenId) -> Veggie;
    fn delete_veggie(&mut self, vid: TokenId);

    fn mint_plant(&mut self, 
                    vsubtype: VeggieSubType,
                    )->Veggie;

    fn get_plant(&self, vid: TokenId) -> Veggie;

    fn delete_plant(&mut self, vid: TokenId);
}

// veggie implementation
//

#[near_bindgen]
impl Veggies for PlantaryContract {
    fn create_veggie(&mut self, 
                    vtype: VeggieType,
                    vsubtype: VeggieSubType,
                    ) -> Veggie {
        // make a local veggie
        let c = Veggie::new(self.gen_token_id(), vtype, vsubtype);
        // record in the static list of veggies
        self.veggies.insert(&c.id, &c);
        // record ownership in the nft structure
        self.token_bank.mint_token(env::predecessor_account_id(), c.id);
        return c;
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

    fn mint_plant(&mut self,
                    vsubtype: VeggieSubType,
                    ) -> Veggie {
        // make sure that only the owner can call this funtion (needed?)
        self.only_owner();
        return self.create_veggie(vtypes::PLANT, vsubtype);
    }

    fn get_plant(&self, vid: TokenId) -> Veggie {
        return self.get_veggie(vid);
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
                    )->Veggie;

    fn get_harvest(self, vid: TokenId) -> Veggie;

    fn delete_harvest(&mut self, vid: TokenId);

    fn harvest_plant(&mut self, parent: &Veggie) -> Veggie;
}

impl Harvests for PlantaryContract {
    fn create_harvest(&mut self,
                    vsubtype: VeggieSubType,
                    ) -> Veggie {
        return self.create_veggie(vtypes::HARVEST, vsubtype);
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
        let mut h = self.create_harvest(htypes::GENERIC);
        h.parent = parent.id;
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
      //      token_to_account: UnorderedMap::new(b"token-belongs-to".to_vec()),
     //       account_to_tokens: UnorderedMap::new(b"account-owns".to_vec()),
      //      account_gives_access: UnorderedMap::new(b"gives-access".to_vec()),
            token_bank: TokenBank::new(),
            owner_id,
            veggies: UnorderedMap::new(b"veggies".to_vec()),
        }
    }

   pub fn gen_token_id(&self) -> TokenId {
        // TODO: ask a pro if this is anything like the right way to get a random tokenID in NEAR!
        // Near provides this vector of random bytes, will it always be 32 bytes long? we only need 8 ...
        let s = env::random_seed();
        let mut id: TokenId = 0;
        for val in s[0..7].iter() {
            id = (id << 8) + (*val as u64);
        }

        // if ever that totally random ID is in already in use, just increment.
        while self.token_bank.token_to_account.get(&id).is_some(){
            id += 1;
        }

        return id; 
    }

    pub fn get_owner_tokens(&self, account_id: &AccountId) -> Vec<TokenId> {
        self.token_bank.get_owner_tokens(&account_id).to_vec()
    }

    /// helper function determining contract ownership
    /// Really these token functions all need some clearer security framework.
    fn only_owner(&mut self) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only contract owner can call this method.");
    }
}

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    use constants::{vtypes, ptypes};
    use crate::token_bank::NEP4;

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
                account_balance: 0,
                account_locked_balance: 0,
                storage_usage,
                attached_deposit: 0,
                prepaid_gas: 10u64.pow(18),
                random_seed: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                is_view: false,
                output_data_receivers: vec![],
                epoch_height: 19,
            }
        }

        #[test]
        fn grant_access() {
            let context = get_context(robert(), 0);
            testing_env!(context);
            let contract = PlantaryContract::new(robert());
            let mut tb = contract.token_bank;
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
            let mut contract = PlantaryContract::new(robert());
            contract.token_bank.revoke_access(joe());
        }

        #[test]
        fn add_revoke_access_and_check() {
            // Joe grants access to Robert
            let mut context = get_context(joe(), 0);
            testing_env!(context);
            let contract = PlantaryContract::new(joe());
            let mut tb = contract.token_bank;
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
            let contract = PlantaryContract::new(robert());
            let mut tb = contract.token_bank;
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
            let contract = PlantaryContract::new(robert());
            let mut tb = contract.token_bank;
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
            let contract = PlantaryContract::new(mike());
            let mut tb = contract.token_bank;
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
            let contract = PlantaryContract::new(mike());
            let mut tb = contract.token_bank;
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
            let contract = PlantaryContract::new(robert());
            let mut tb = contract.token_bank;
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
            let contract = PlantaryContract::new(mike());
            let mut tb = contract.token_bank;
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
            let contract = PlantaryContract::new(robert());
            let mut tb = contract.token_bank;
            let token_id = 19u64;
            tb.mint_token(robert(), token_id);

            // Robert transfers the token to Joe
            tb.transfer(joe(), token_id.clone());

            // Check new owner
            let owner = tb.get_token_owner(token_id.clone());
            assert_eq!(joe(), owner, "Token was not transferred after transfer call with escrow.");
        }

        #[test]
        #[should_panic(
            expected = r#"Veggie does not exist."#
        )]
        fn create_delete_veggie() {
            testing_env!(get_context(robert(), 0));
            let mut contract = PlantaryContract::new(robert());

                // create
            let v = contract.create_veggie(vtypes::PLANT, ptypes::GENERIC);
                // inspect?
            assert_eq!(v.vsubtype, ptypes::GENERIC, "subtype not found.");
                // find?
            let vid = v.id;
                // confirm
            let _foundv = contract.get_veggie(vid); // should not panic
            /*
            // TODO: how to compare two objects?
            assert_eq!(v, _foundv, "veggie did not fetch right");
            */
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
            let p = contract.mint_plant(ptypes::GENERIC);
                // inspect
            assert_eq!(p.vtype, vtypes::PLANT, "vtype not saved");
            assert_eq!(p.vsubtype, ptypes::GENERIC, "vsubtype not saved");
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

        #[test]
        fn harvest_plant(){
            testing_env!(get_context(robert(), 0));
            let mut contract = PlantaryContract::new(robert());

                // create
            let p = contract.mint_plant(ptypes::GENERIC);
            let h = contract.harvest_plant(&p);
                // inspect
            assert_eq!(p.id, h.parent, "parentage suspect");
        }

        #[test]
        fn test_gen_id(){
            testing_env!(get_context(robert(), 0));
            let contract = PlantaryContract::new(robert());

            let _randid = contract.gen_token_id();
        }

        #[test]
        fn mint_burn_token(){
            testing_env!(get_context(robert(), 0));
            let contract = PlantaryContract::new(robert());
            let mut tb = contract.token_bank;
            let token_id = 19u64;

            // mint
            tb.mint_token(robert(), token_id);
            
            // burn
            tb.burn_token(token_id);
        }

        #[test]
        #[should_panic(
            expected = r#"not yours to burn"#
        )]
        fn cant_burn_mine(){
            testing_env!(get_context(robert(), 0));
            let contract = PlantaryContract::new(robert());
            let mut tb = contract.token_bank;
            let token_id = 19u64;
            // mint
            tb.mint_token(mike(), token_id);
            
            // burn (as robert)
            tb.burn_token(token_id);
        }

        #[test]
        fn get_owner_tokens(){
            testing_env!(get_context(robert(), 0));
            let contract = PlantaryContract::new(robert());
            let mut tb = contract.token_bank;
            let mut token_id = 19u64;

            // mint 3
            tb.mint_token(robert(), token_id);
            token_id += 1;
            tb.mint_token(robert(), token_id);
            token_id += 1;
            tb.mint_token(robert(), token_id);

            // get them all
            let mut tokens = tb.get_owner_tokens(&robert());

            // should be 3
            //
            assert_eq!(tokens.len(), 3, "didn't get all 3 tokens");
            // burn 1
            token_id = 19u64;
            tb.burn_token(token_id);
            // get them all 
            tokens = tb.get_owner_tokens(&robert());
            // should be 2
            assert_eq!(tokens.len(), 2, "didn't get both tokens");
            // burn both
            token_id += 1;
            tb.burn_token(token_id);
            token_id += 1;
            tb.burn_token(token_id);
            // get them all
            tokens = tb.get_owner_tokens(&robert());
            // should be empty
            assert_eq!(tokens.len(), 0, "why did i get tokens?");
        }
}
