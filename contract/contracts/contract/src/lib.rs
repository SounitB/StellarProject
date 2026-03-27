#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short, Map, String};

#[contract]
pub struct CarbonTracker;

#[contractimpl]
impl CarbonTracker {
    // Add carbon emission entry for a user
    pub fn add_emission(env: Env, user: String, amount: u32) {
        let key = Symbol::new(&env, "emissions");

        let mut emissions: Map<String, u32> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        let current = emissions.get(user.clone()).unwrap_or(0);
        emissions.set(user, current + amount);

        env.storage().instance().set(&key, &emissions);
    }

    // Get total emissions for a user
    pub fn get_emission(env: Env, user: String) -> u32 {
        let key = symbol_short!("emissions");

        let emissions: Map<String, u32> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        emissions.get(user).unwrap_or(0)
    }

    // Reset emissions for a user
    pub fn reset_emission(env: Env, user: String) {
        let key = symbol_short!("emissions");

        let mut emissions: Map<String, u32> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        emissions.remove(user);

        env.storage().instance().set(&key, &emissions);
    }
}
