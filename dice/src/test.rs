#![cfg(test)]

use super::*;
use soroban_sdk::Env;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, DiceContract);
    let die = DiceContractClient::new(&env, &contract_id);

    assert!(die.roll() >= 1 && die.roll() <= 6);
}
