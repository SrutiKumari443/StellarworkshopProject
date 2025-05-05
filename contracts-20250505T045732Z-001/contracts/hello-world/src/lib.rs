#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct CreditRecord {
    pub id: u64,
    pub org_name: String,
    pub credits_earned: u64,
    pub credits_used: u64,
}

#[contracttype]
pub enum CreditBook {
    Record(u64),
}

const CREDIT_COUNT: Symbol = symbol_short!("C_COUNT");

#[contract]
pub struct CarbonCreditTracker;

#[contractimpl]
impl CarbonCreditTracker {
    pub fn log_credits(env: Env, org_name: String, earned: u64) -> u64 {
        let mut count = env.storage().instance().get(&CREDIT_COUNT).unwrap_or(0);
        count += 1;

        let record = CreditRecord {
            id: count,
            org_name,
            credits_earned: earned,
            credits_used: 0,
        };

        env.storage().instance().set(&CreditBook::Record(count), &record);
        env.storage().instance().set(&CREDIT_COUNT, &count);

        count
    }

    pub fn use_credits(env: Env, id: u64, amount: u64) {
        let key = CreditBook::Record(id);
        let mut record: CreditRecord = env.storage().instance().get(&key).expect("Record not found");

        if record.credits_used + amount > record.credits_earned {
            panic!("Not enough credits!");
        }

        record.credits_used += amount;
        env.storage().instance().set(&key, &record);
    }

    pub fn get_credit_record(env: Env, id: u64) -> CreditRecord {
        env.storage().instance().get(&CreditBook::Record(id)).unwrap_or(CreditRecord {
            id: 0,
            org_name: String::from_str(&env, "Not Found"),
            credits_earned: 0,
            credits_used: 0,
        })
    }

    pub fn total_records(env: Env) -> u64 {
        env.storage().instance().get(&CREDIT_COUNT).unwrap_or(0)
    }
}
