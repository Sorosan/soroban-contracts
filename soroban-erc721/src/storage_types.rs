use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Balance(Address),
    Minted(Address),
    Admin,
    Name,
    Symbol,
    URI(u128),
    Approval(ApprovalKey),
    Owner(u128),
    Supply,
}

#[derive(Clone)]
#[contracttype]
pub enum UserDataKey {
    TokenOwner(u128),
}

#[derive(Clone)]
#[contracttype]
pub struct ApprovalKey {
    owner: Address,
    spender: Address,
}