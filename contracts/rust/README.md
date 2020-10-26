Plantary smart contract
========================

This contract manages ownership and game dynamics of Plantary NFTs.
Its main pieces of state are a NEP4-compatible NFT registry (TokenBank), 
a ledger of metadata attached to the NFTs (Veggies),
and a pool of potential-metadata awaiting union with newly-minted tokens (Seeds).

Enhanced NEP#4 implementation
============================

Plantary includes TokenBank, a fork of NEAR's reference inmplementation of the NEP4 NFT spec: https://github.com/near-examples/NFT.
See NEAR's repo for more info on the implentation: https://github.com/near-examples/NFT/tree/master/contracts/rust

Enhancements beyond NEP#4
==========================================
* Tokens and their metadata can be queried by owner, by type, or in sum
* Queries that return tokens are paged for big-data compatibility
* Tokens can be both minted and burned

Some limitations of the current implementation
===========================================================
* Only the token owner can mint tokens.
* You cannot give another account escrow access to a limited set of your tokens; an escrow must be trusted with all of your tokens or none at all
* Usability issues: some functions (e.g. `revoke_access`, `transfer`, `get_token_owner`) do not verify that they were given sensible inputs; if given non-existent keys, the errors they throw will not be very useful
