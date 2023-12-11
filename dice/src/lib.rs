#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct DiceContract;

#[contractimpl]
impl DiceContract {
    pub fn roll(env: Env) -> u64 {
        env.prng().u64_in_range(1..=6)
    }
}

mod test;
