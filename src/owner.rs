use soroban_sdk::{Address, Env};
use crate::storage_types::{DataKey, UserDataKey};

pub fn read_owner(e: &Env, id: u128) -> Address {
    let key = UserDataKey::TokenOwner(id);
    e.storage()
        .persistent()
        .get(&key)
        .unwrap_or_else(|| panic!("NFT doesn't exist"))
}

pub fn write_owner(e: &Env, id: u128, owner: &Address) {
    let key = UserDataKey::TokenOwner(id);
    e.storage().persistent().set(&key, owner);
    e.storage().persistent().set(&DataKey::Minted(owner.clone()), &id);
}

pub fn remove_owner(e: &Env, id: u128, owner: &Address) {
    let key = UserDataKey::TokenOwner(id);
    e.storage().persistent().set(&key, owner);
    e.storage().persistent().set(&DataKey::Minted(owner.clone()), &id);
}

pub fn has_owner(e: &Env, id: u128) -> bool {
    e.storage().persistent().has(&UserDataKey::TokenOwner(id))
}