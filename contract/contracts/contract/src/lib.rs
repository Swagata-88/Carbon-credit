#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, Address, Map};

#[contract]
pub struct CarbonCreditContract;

#[contractimpl]
impl CarbonCreditContract {

    // Initialize storage
    pub fn init(env: Env) {
        let balances: Map<Address, i128> = Map::new(&env);
        env.storage().instance().set(&Symbol::short("BAL"), &balances);
    }

    // Mint carbon credits
    pub fn mint(env: Env, to: Address, amount: i128) {
        to.require_auth();

        let mut balances: Map<Address, i128> =
            env.storage().instance().get(&Symbol::short("BAL")).unwrap();

        let current = balances.get(to.clone()).unwrap_or(0);
        balances.set(to, current + amount);

        env.storage().instance().set(&Symbol::short("BAL"), &balances);
    }

    // Transfer credits
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        let mut balances: Map<Address, i128> =
            env.storage().instance().get(&Symbol::short("BAL")).unwrap();

        let from_balance = balances.get(from.clone()).unwrap_or(0);
        if from_balance < amount {
            panic!("Insufficient balance");
        }

        balances.set(from.clone(), from_balance - amount);

        let to_balance = balances.get(to.clone()).unwrap_or(0);
        balances.set(to, to_balance + amount);

        env.storage().instance().set(&Symbol::short("BAL"), &balances);
    }

    // Check balance
    pub fn balance(env: Env, user: Address) -> i128 {
        let balances: Map<Address, i128> =
            env.storage().instance().get(&Symbol::short("BAL")).unwrap();

        balances.get(user).unwrap_or(0)
    }
}