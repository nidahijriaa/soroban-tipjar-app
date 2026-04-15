#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env, String,
};

// Melakukan test disini
fn create_test_env() -> (Env, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, TipJarContract);
    let owner = Address::generate(&env);
    let tipper = Address::generate(&env);

    (env, contract_id, owner, tipper)
}

#[test]
fn test_initialize() {
    let (env, contract_id, owner, _) = create_test_env();
    let client = TipJarContractClient::new(&env, &contract_id);

    client.initialize(&owner);

    assert_eq!(client.get_owner(), owner);
    assert_eq!(client.get_total(), 0);
    assert_eq!(client.get_tips().len(), 0);
}

#[test]
#[should_panic(expected = "Already initialized")]
fn test_double_initialize() {
    let (env, contract_id, owner, _) = create_test_env();
    let client = TipJarContractClient::new(&env, &contract_id);

    client.initialize(&owner);
    client.initialize(&owner); // harus panic
}

#[test]
fn test_send_tip() {
    let (env, contract_id, owner, tipper) = create_test_env();

    // Setup token
    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_client = soroban_sdk::token::StellarAssetClient::new(&env, &token_id.address());

    // Mint tokens ke tipper
    token_client.mint(&tipper, &1000_0000000i128);

    let client = TipJarContractClient::new(&env, &contract_id);
    client.initialize(&owner);

    let message = String::from_str(&env, "Great content! Keep it up!");

    client.send_tip(&tipper, &token_id.address(), &100_0000000i128, &message);

    let tips = client.get_tips();
    assert_eq!(tips.len(), 1);
    assert_eq!(tips.get(0).unwrap().amount, 100_0000000i128);
    assert_eq!(client.get_total(), 100_0000000i128);
}

#[test]
#[should_panic(expected = "Amount must be positive")]
fn test_invalid_amount() {
    let (env, contract_id, owner, tipper) = create_test_env();
    let token_id = env.register_stellar_asset_contract_v2(Address::generate(&env));

    let client = TipJarContractClient::new(&env, &contract_id);
    client.initialize(&owner);

    let message = String::from_str(&env, "test");
    client.send_tip(&tipper, &token_id.address(), &0i128, &message);
}

#[test]
fn test_multiple_tips() {
    let (env, contract_id, owner, _) = create_test_env();

    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_client = soroban_sdk::token::StellarAssetClient::new(&env, &token_id.address());

    let tipper1 = Address::generate(&env);
    let tipper2 = Address::generate(&env);
    token_client.mint(&tipper1, &500_0000000i128);
    token_client.mint(&tipper2, &500_0000000i128);

    let client = TipJarContractClient::new(&env, &contract_id);
    client.initialize(&owner);

    client.send_tip(
        &tipper1,
        &token_id.address(),
        &100_0000000i128,
        &String::from_str(&env, "First tip!"),
    );
    client.send_tip(
        &tipper2,
        &token_id.address(),
        &200_0000000i128,
        &String::from_str(&env, "Keep going!"),
    );

    assert_eq!(client.get_tips().len(), 2);
    assert_eq!(client.get_total(), 300_0000000i128);
}