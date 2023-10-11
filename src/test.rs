// 1. ERC721 function currently available for NFT: mint, transfer, owner_of
#![cfg(test)]

extern crate std;

use crate::{contract::ErasNftContract, ErasNftContractClient};
use soroban_sdk::{
    testutils::{Address as _, Logs},
    Address, Env, IntoVal,
};

fn create_token<'a>(env: &Env, admin: &Address) -> ErasNftContractClient<'a> {
    let token = ErasNftContractClient::new(env, &env.register_contract(None, ErasNftContract {}));
    token.initialize(admin, &"Sorosan NFT".into_val(env), &"Sorosan".into_val(env));
    token
}

#[test]
fn test_mint() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let user1 = Address::random(&env);
    let user2 = Address::random(&env);
    let nft = create_token(&env, &admin);

    nft.mint(&user1, &1);
    nft.mint(&user2, &2);
    nft.mint(&user1, &3);

    std::println!("{}", env.logs().all().join("\n"));
}

#[test]
fn test_owner_of() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let user1 = Address::random(&env);
    let nft = create_token(&env, &admin);

    nft.mint(&user1, &1);

    assert_eq!(nft.owner_of(&1), user1);
    std::println!("{}", env.logs().all().join("\n"));
}

#[test]
fn test_transfer() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let user1 = Address::random(&env);
    let user2 = Address::random(&env);
    let nft = create_token(&env, &admin);
    nft.mint(&user1, &1);

    eras_token.transfer(&swift_fan_1, &swift_fan_2, &1);
    eras_token.transfer(&swift_fan_2, &swift_fan_1, &1);

    assert_eq!(eras_token.owner_of(&1), swift_fan_1);
    assert_eq!(eras_token.owner_of(&1), swift_fan_2);
    assert_eq!(eras_token.owner_of(&1), swift_fan_1);
    std::println!("{}", env.logs().all().join("\n"));
}

#[test]
#[should_panic(expected = "ID already minted")]
fn seat_already_taken() {
    let env = Env::default();
    env.mock_all_auths();
    let user1 = Address::random(&env);
    let user2 = Address::random(&env);
    let nft = create_token(&env, &admin);
    nft.mint(&user1, &1);

    nft.mint(&user2, &1);
    std::println!("{}", env.logs().all().join("\n"));
}