use crate::storage_types::{DataKey};
use soroban_sdk::{Env, String};

pub fn read_name(e: &Env) -> String {
    let key = DataKey::Name;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_name(e: &Env, name: &String) {
    let key = DataKey::Name;
    e.storage().instance().set(&key, name);
}

pub fn read_symbol(e: &Env) -> String {
    let key = DataKey::Name;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_symbol(e: &Env, symbol: &String) {
    let key = DataKey::Symbol;
    e.storage().instance().set(&key, symbol);
}

pub fn read_token_uri(e: &Env, id: u128) -> String {
    let key = DataKey::URI(id);
    e.storage().instance().get(&key).unwrap()
}

pub fn write_token_uri(e: &Env, id: u128, uri: &String) {
    let key = DataKey::URI(id);
    e.storage().instance().set(&key, uri);
}