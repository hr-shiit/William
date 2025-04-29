#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, Symbol,
};

use crate::WilliamContract;

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, WilliamContract);
    let client = WilliamContract::new(&env, &contract_id);

    let owner = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&owner);

    // Test owner is set correctly
    assert_eq!(client.get_owner(), owner);
    
    // Test initial state
    assert_eq!(client.get_is_dead(), false);
    assert_eq!(client.get_beneficiary_addresses().len(), 0);
    assert_eq!(client.get_contract_balance(), 0);
}

#[test]
fn test_store_beneficiary() {
    let env = Env::default();
    let contract_id = env.register_contract(None, WilliamContract);
    let client = WilliamContract::new(&env, &contract_id);

    let owner = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&owner);

    let beneficiary = Address::generate(&env);
    client.store_beneficiary(&beneficiary, &50);

    // Test beneficiary is stored correctly
    assert_eq!(client.get_beneficiary_percentage(&beneficiary), 50);
    assert_eq!(client.get_beneficiary_addresses().len(), 1);
}

#[test]
#[should_panic(expected = "Percentage must be between 1 and 100")]
fn test_store_beneficiary_invalid_percentage() {
    let env = Env::default();
    let contract_id = env.register_contract(None, WilliamContract);
    let client = WilliamContract::new(&env, &contract_id);

    let owner = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&owner);

    let beneficiary = Address::generate(&env);
    client.store_beneficiary(&beneficiary, &101); // Should panic
}

#[test]
fn test_delete_beneficiary() {
    let env = Env::default();
    let contract_id = env.register_contract(None, WilliamContract);
    let client = WilliamContract::new(&env, &contract_id);

    let owner = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&owner);

    let beneficiary = Address::generate(&env);
    client.store_beneficiary(&beneficiary, &50);
    assert_eq!(client.get_beneficiary_addresses().len(), 1);

    client.delete_beneficiary(&beneficiary);
    assert_eq!(client.get_beneficiary_percentage(&beneficiary), 0);
    assert_eq!(client.get_beneficiary_addresses().len(), 0);
}

#[test]
fn test_death_check() {
    let env = Env::default();
    let contract_id = env.register_contract(None, WilliamContract);
    let client = WilliamContract::new(&env, &contract_id);

    let owner = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&owner);

    client.death_check(&1);
    assert_eq!(client.get_is_dead(), true);

    client.death_check(&0);
    assert_eq!(client.get_is_dead(), false);
}
