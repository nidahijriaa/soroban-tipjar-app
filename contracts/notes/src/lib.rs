#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, String, Vec, token,
};

// Data structures untuk menyimpan tip
#[contracttype]
#[derive(Clone)]
pub struct Tip {
    pub from: Address,
    pub amount: i128,
    pub message: String,
    pub timestamp: u64,
}

#[contracttype]
pub enum DataKey {
    Owner,
    Tips,
    TotalReceived,
}

#[contract]
pub struct TipJarContract;

#[contractimpl]
impl TipJarContract {
    /// Initialize contract dengan owner (kreator)
    pub fn initialize(env: Env, owner: Address) {
        if env.storage().instance().has(&DataKey::Owner) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage().instance().set(&DataKey::TotalReceived, &0i128);
        let tips: Vec<Tip> = Vec::new(&env);
        env.storage().instance().set(&DataKey::Tips, &tips);
    }

    /// Kirim tip XLM ke kreator
    pub fn send_tip(
        env: Env,
        from: Address,
        token_address: Address,
        amount: i128,
        message: String,
    ) {
        from.require_auth();

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();

        // Transfer token dari pengirim ke owner
        let token_client = token::Client::new(&env, &token_address);
        token_client.transfer(&from, &owner, &amount);

        // Simpan record tip
        let tip = Tip {
            from: from.clone(),
            amount,
            message,
            timestamp: env.ledger().timestamp(),
        };

        let mut tips: Vec<Tip> = env
            .storage()
            .instance()
            .get(&DataKey::Tips)
            .unwrap_or(Vec::new(&env));
        tips.push_back(tip);
        env.storage().instance().set(&DataKey::Tips, &tips);

        // Update total
        let mut total: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalReceived)
            .unwrap_or(0);
        total += amount;
        env.storage().instance().set(&DataKey::TotalReceived, &total);

        // Emit event
        env.events().publish(
            (symbol_short!("tip_sent"), from),
            amount,
        );
    }

    /// Ambil semua tips
    pub fn get_tips(env: Env) -> Vec<Tip> {
        env.storage()
            .instance()
            .get(&DataKey::Tips)
            .unwrap_or(Vec::new(&env))
    }

    /// Ambil total yang diterima
    pub fn get_total(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::TotalReceived)
            .unwrap_or(0)
    }

    /// Ambil alamat owner
    pub fn get_owner(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Owner).unwrap()
    }
}