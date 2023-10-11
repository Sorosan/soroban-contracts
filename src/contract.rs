use crate::admin::{has_admin, read_admin, write_admin};
use crate::owner::{read_owner, write_owner, has_owner};
use crate::metadata::{
    write_name, read_name, 
    write_symbol, read_symbol, 
    read_token_uri, write_token_uri
};
use crate::balance::{read_supply, increment_supply};
use soroban_sdk::{
    contract, contractimpl, log, Address, Env, String
};
use crate::storage_types::{DataKey, UserDataKey};

#[contract]
pub struct NonFungibleToken;

#[contractimpl]
impl NonFungibleToken {
    pub fn initialize(env: Env, admin: Address, name: String, symbol: String) {
        if has_admin(&env) {
            panic!("already initialized")
        }
        
        write_admin(&env, &admin);
        write_name(&env, &name);
        write_symbol(&env, &symbol);
    }

    pub fn name(env: Env) -> String {
        read_name(&env)
    }

    pub fn symbol(env: Env) -> String {
        read_symbol(&env)
    }

    pub fn token_uri(env: Env, id: u128) -> String {
        read_token_uri(&env, id)
    }
    pub fn supply(env: Env) -> u128 {
        read_supply(&env)
    }
    
    pub fn mint(env: Env, to: Address, id: u128, uri: String) {
        let admin = read_admin(&env);
        admin.require_auth();

        // Panic if NFT exist
        if has_owner(&env, id) {
            panic!("ID already minted");
        }

        // Safe Mint
        if !has_owner(&env, id) {
            increment_supply(&env);
            write_owner(&env, id, &to);
            write_token_uri(&env, id, &uri);
        }
    }

    pub fn owner_of(env: Env, id: u128) -> Address{
        read_owner(&env, id)
    }

    pub fn transfer(env: Env, from: Address, to: Address, id: u128) {
        from.require_auth();

        // Valid NFT owners
        if env.storage().persistent().has(&DataKey::Minted(to.clone())) {
            panic!("this receiver already has a nft ticket to eras tour");
        }

        if env.storage().persistent().has(&DataKey::Minted(from.clone())) {
            env.storage().persistent().remove(&DataKey::Minted(from));
        }

        write_owner(&env, id, &to);
    }
}