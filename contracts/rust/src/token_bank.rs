#![deny(warnings)]

/// adapted from https://github.com/near-examples/NFT by mykle
/// Implements blockchain ledger for plants and their fruit

use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::collections::UnorderedSet;
use near_sdk::{env, AccountId};

/// This trait provides the baseline of functions as described at:
/// https://github.com/nearprotocol/NEPs/blob/nep-4/specs/Standards/Tokens/NonFungibleToken.md
pub trait NEP4 {
    // Grant the access to the given `accountId` for the given `tokenId`.
    // Requirements:
    // * The caller of the function (`predecessor_id`) should have access to the token.
    fn grant_access(&mut self, escrow_account_id: AccountId);

    // Revoke the access to the given `accountId` for the given `tokenId`.
    // Requirements:
    // * The caller of the function (`predecessor_id`) should have access to the token.
    fn revoke_access(&mut self, escrow_account_id: AccountId);

    // Transfer the given `tokenId` to the given `accountId`. Account `accountId` becomes the new owner.
    // Requirements:
    // * The caller of the function (`predecessor_id`) should have access to the token.
    fn transfer_from(&mut self, owner_id: AccountId, new_owner_id: AccountId, token_id: TokenId); 

    // Transfer the given `tokenId` to the given `accountId`. Account `accountId` becomes the new owner.
    // Requirements:
    // * The caller of the function (`predecessor_id`) should be the owner of the token. Callers who have
    // escrow access should use transfer_from.
    fn transfer(&mut self, new_owner_id: AccountId, token_id: TokenId); 

    // Returns `true` or `false` based on caller of the function (`predecessor_id) having access to the token
    fn check_access(&self, account_id: &AccountId) -> bool;

    // Get an individual owner by given `tokenId`.
    fn get_token_owner(&self, token_id: TokenId) -> String;
}

/// The token ID type is also defined in the NEP
pub type TokenId = u64;
pub type TokenSet = UnorderedSet<TokenId>;
pub type AccountIdHash = Vec<u8>;

// Begin implementation
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TokenBank {
    // ownership structure:
    pub token_to_account: UnorderedMap<TokenId, AccountId>,
    pub account_to_tokens: UnorderedMap<AccountId, TokenSet>,
    pub account_gives_access: UnorderedMap<AccountIdHash, UnorderedSet<AccountIdHash>>, // Vec<u8> is sha256 of account, makes it safer and is how fungible token also works
}

impl TokenBank {
    pub fn new() -> Self {
        Self {
            token_to_account: UnorderedMap::new(b"token-belongs-to".to_vec()),
            account_to_tokens: UnorderedMap::new(b"account-owns".to_vec()),
            account_gives_access: UnorderedMap::new(b"gives-access".to_vec()),
        }
    }

    // Non-NEP handy token functions:
    //
    // Gets list of tokens by owner
    pub fn get_owner_tokens(&self, account_id: &AccountId) -> TokenSet {
        match self.account_to_tokens.get(&account_id) {
            Some(owner_tokens) => owner_tokens,
            None => UnorderedSet::new(b"owner-tokens-set".to_vec())
        }
    }
    
    /// Creates a token for owner_id, doesn't use autoincrement, fails if id is taken
    pub fn mint_token(&mut self, owner_id: String, token_id: TokenId) {
        // Since Map doesn't have `contains` we use match
        let token_check = self.token_to_account.get(&token_id);
        if token_check.is_some() {
            env::panic(b"Token ID already exists.")
        }

        // No token with that ID exists. mint and add token to data structures
        let mut new_owner_tokens = self.get_owner_tokens(&owner_id);
        new_owner_tokens.insert(&token_id);
        self.account_to_tokens.insert(&owner_id, &new_owner_tokens);
        self.token_to_account.insert(&token_id, &owner_id);


    }

    // burns a token
    pub fn burn_token(&mut self, token_id: TokenId) {
        let owner_id = self.get_token_owner(token_id);
        let predecessor = env::predecessor_account_id();
        if predecessor != owner_id {
            env::panic(b"not yours to burn")
        }

        let mut owner_tokens = self.get_owner_tokens(&owner_id);
        owner_tokens.remove(&token_id);
        self.account_to_tokens.insert(&owner_id, &owner_tokens);
        self.token_to_account.remove(&token_id);
    }

}

impl Default for TokenBank {
    fn default() -> Self {
        panic!("tokenbank should be initialized before usage")
    }
}

impl NEP4 for TokenBank {
    fn grant_access(&mut self, escrow_account_id: AccountId) {
        let escrow_hash = env::sha256(escrow_account_id.as_bytes());
        let predecessor = env::predecessor_account_id();
        let predecessor_hash = env::sha256(predecessor.as_bytes());

        let mut access_set = match self.account_gives_access.get(&predecessor_hash) {
            Some(existing_set) => {
                existing_set
            },
            None => {
                UnorderedSet::new(b"new-access-set".to_vec())
            }
        };
        access_set.insert(&escrow_hash);
        self.account_gives_access.insert(&predecessor_hash, &access_set);
    }

    fn revoke_access(&mut self, escrow_account_id: AccountId) {
        let predecessor = env::predecessor_account_id();
        let predecessor_hash = env::sha256(predecessor.as_bytes());
        let mut existing_set = match self.account_gives_access.get(&predecessor_hash) {
            Some(existing_set) => existing_set,
            None => env::panic(b"Access does not exist.")
        };
        let escrow_hash = env::sha256(escrow_account_id.as_bytes());
        if existing_set.contains(&escrow_hash) {
            existing_set.remove(&escrow_hash);
            self.account_gives_access.insert(&predecessor_hash, &existing_set);
            env::log(b"Successfully removed access.")
        } else {
            env::panic(b"Did not find access for escrow ID.")
        }
    }

    fn transfer(&mut self, new_owner_id: AccountId, token_id: TokenId) {
        let token_owner_account_id = self.get_token_owner(token_id);
        let predecessor = env::predecessor_account_id();
        if predecessor != token_owner_account_id {
            env::panic(b"Attempt to call transfer on tokens belonging to another account.")
        }

        let mut new_owner_tokens = self.get_owner_tokens(&new_owner_id);
        let mut prev_owner_tokens = self.get_owner_tokens(&token_owner_account_id);

        new_owner_tokens.insert(&token_id);

        // Q: if owner_tokens is now empty, would it be more NEAR-optimal to delete it from the map?
        prev_owner_tokens.remove(&token_id); 

        // Q: In NEAR, is a transaction guaranteed around a smart method call?
        // Cuz these three need to be a transaction:
        self.token_to_account.insert(&token_id, &new_owner_id);
        self.account_to_tokens.insert(&new_owner_id, &new_owner_tokens);
        self.account_to_tokens.insert(&token_owner_account_id, &prev_owner_tokens);
    }

    fn transfer_from(&mut self, owner_id: AccountId, new_owner_id: AccountId, token_id: TokenId) {
        let token_owner_account_id = self.get_token_owner(token_id);
        if owner_id != token_owner_account_id {
            env::panic(b"Attempt to transfer a token from wrong owner.")
        }

        if !self.check_access(&token_owner_account_id) {
            env::panic(b"Attempt to transfer a token with no access.")
        }

        let mut new_owner_tokens = self.get_owner_tokens(&new_owner_id);
        let mut prev_owner_tokens = self.get_owner_tokens(&token_owner_account_id);

        new_owner_tokens.insert(&token_id);
        prev_owner_tokens.remove(&token_id); 
        self.token_to_account.insert(&token_id, &new_owner_id);
        self.account_to_tokens.insert(&new_owner_id, &new_owner_tokens);
        self.account_to_tokens.insert(&token_owner_account_id, &prev_owner_tokens);
    }

    fn check_access(&self, account_id: &AccountId) -> bool {
        let account_hash = env::sha256(account_id.as_bytes());
        let predecessor = env::predecessor_account_id();
        if predecessor == *account_id {
            return true;
        }
        match self.account_gives_access.get(&account_hash) {
            Some(access) => {
                let predecessor = env::predecessor_account_id();
                let predecessor_hash = env::sha256(predecessor.as_bytes());
                access.contains(&predecessor_hash)
            },
            None => false
        }
    }

    fn get_token_owner(&self, token_id: TokenId) -> String {
        match self.token_to_account.get(&token_id) {
            Some(owner_id) => owner_id,
            None => env::panic(b"No owner of the token ID specified")
        }
    }
    
}

