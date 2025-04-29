#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, 
    Address, Env, Map, Symbol, Vec, 
    events, log
};

#[contracttype]
pub enum DataKey {
    Owner,
    Beneficiary(Address),
    BeneficiaryAddresses,
    TotalDeposited,
    IsDead,
}

#[contract]
pub struct WilliamContract;

#[contractimpl]
impl WilliamContract {
    // Initialize the contract with the owner (similar to constructor)
    pub fn initialize(env: Env, owner: Address) {
        // Verify contract is not already initialized
        if let None = env.storage().persistent().get::<_, Address>(&DataKey::Owner) {
            owner.require_auth();
            env.storage().persistent().set(&DataKey::Owner, &owner);
            
            // Initialize other state
            let beneficiary_addresses: Vec<Address> = Vec::new(&env);
            env.storage().persistent().set(&DataKey::BeneficiaryAddresses, &beneficiary_addresses);
            env.storage().persistent().set(&DataKey::TotalDeposited, &0_i128);
            env.storage().persistent().set(&DataKey::IsDead, &false);
        }
    }
    
    // Delete a beneficiary from the contract
    pub fn delete_beneficiary(env: Env, ben_add: Address) {
        // Only owner can call this function
        let owner = env.storage().persistent().get::<_, Address>(&DataKey::Owner).unwrap();
        owner.require_auth();
        
        // Check if the beneficiary exists
        let existing_percentage = env.storage().persistent().get::<_, i128>(&DataKey::Beneficiary(ben_add.clone()));
        assert!(existing_percentage.is_some(), "Beneficiary does not exist");
        
        // Remove from storage
        env.storage().persistent().remove(&DataKey::Beneficiary(ben_add.clone()));
        
        // Remove from beneficiary addresses list
        let mut ben_addresses = env.storage().persistent().get::<_, Vec<Address>>(&DataKey::BeneficiaryAddresses)
            .unwrap_or(Vec::new(&env));
        
        // Find the index of the beneficiary to remove
        let mut index_to_remove: Option<u32> = None;
        for i in 0..ben_addresses.len() {
            if ben_addresses.get(i).unwrap() == ben_add {
                index_to_remove = Some(i);
                break;
            }
        }
        
        // Remove the beneficiary if found
        if let Some(index) = index_to_remove {
            // In Soroban's Vec, we can remove by swapping with the last element and popping
            if index < ben_addresses.len() - 1 {
                let last = ben_addresses.get(ben_addresses.len() - 1).unwrap();
                ben_addresses.set(index, last);
            }
            ben_addresses.pop_back();
            env.storage().persistent().set(&DataKey::BeneficiaryAddresses, &ben_addresses);
        }
        
        // Emit deletion event
        emit_beneficiary_deleted(&env, &ben_add);
    }

    // Store beneficiary with their percentage
    pub fn store_beneficiary(env: Env, ben_add: Address, percentage: i128) {
        // Only owner can call this function
        let owner = env.storage().persistent().get::<_, Address>(&DataKey::Owner).unwrap();
        owner.require_auth();
        
        // Validation checks
        assert!(percentage > 0 && percentage <= 100, "Percentage must be between 1 and 100");
        
        // Store the beneficiary percentage
        env.storage().persistent().set(&DataKey::Beneficiary(ben_add.clone()), &percentage);
        
        // Add to beneficiary addresses if not already present
        let mut ben_addresses = env.storage().persistent().get::<_, Vec<Address>>(&DataKey::BeneficiaryAddresses).unwrap();
        let mut found = false;
        for i in 0..ben_addresses.len() {
            if ben_addresses.get(i).unwrap() == ben_add {
                found = true;
                break;
            }
        }
        if !found {
            ben_addresses.push_back(ben_add.clone());
            env.storage().persistent().set(&DataKey::BeneficiaryAddresses, &ben_addresses);
        }
        
        // Emit event
        emit_beneficiary_added(&env, &ben_add, &percentage);
    }

    // Deposit tokens to the contract
    pub fn deposit_core(env: Env, sender: Address, amount: i128) {
        sender.require_auth();
        
        // Basic validation
        assert!(amount > 0, "Must send tokens");
        
        // In Soroban, token transfers are handled differently than in Solidity
        // This is a placeholder for the actual token transfer logic
        // Typically you would use a token client to transfer tokens to the contract
        
        // Update total deposited
        let total_deposited = env.storage().persistent().get::<_, i128>(&DataKey::TotalDeposited).unwrap_or(0);
        env.storage().persistent().set(&DataKey::TotalDeposited, &(total_deposited + amount));
        
        // Emit event
        emit_core_deposited(&env, &sender, &amount);
    }

    // Get contract balance (in Soroban this would be replaced with token-specific queries)
    pub fn get_contract_balance(env: Env) -> i128 {
        // This is a placeholder - in a real implementation, you would use token client
        // to check the contract's token balance
        env.storage().persistent().get::<_, i128>(&DataKey::TotalDeposited).unwrap_or(0)
    }

    // Set death status
    pub fn death_check(env: Env, death_check: u32) {
        // Only owner can call this function
        let owner = env.storage().persistent().get::<_, Address>(&DataKey::Owner).unwrap();
        owner.require_auth();
        
        assert!(death_check == 0 || death_check == 1, "Input must be 0 or 1");
        
        let is_dead = death_check == 1;
        env.storage().persistent().set(&DataKey::IsDead, &is_dead);
        
        // Emit event
        emit_death_status_changed(&env, &is_dead);
    }

    // Transfer to beneficiaries
    pub fn ben_transfer(env: Env) {
        // In Soroban, we would want to verify the death status
        let is_dead = env.storage().persistent().get::<_, bool>(&DataKey::IsDead).unwrap_or(false);
        assert!(is_dead, "Owner must be marked as deceased");
        
        let ben_addresses = env.storage().persistent().get::<_, Vec<Address>>(&DataKey::BeneficiaryAddresses).unwrap();
        assert!(!ben_addresses.is_empty(), "No beneficiaries defined");
        
        let total_deposited = env.storage().persistent().get::<_, i128>(&DataKey::TotalDeposited).unwrap_or(0);
        assert!(total_deposited > 0, "No funds to transfer");
        
        // Calculate total percentage to ensure it's 100%
        let mut total_percentage: i128 = 0;
        for i in 0..ben_addresses.len() {
            let addr = ben_addresses.get(i).unwrap();
            let percentage = env.storage().persistent().get::<_, i128>(&DataKey::Beneficiary(addr.clone())).unwrap_or(0);
            total_percentage += percentage;
        }
        assert!(total_percentage == 100, "Total percentage must equal 100");
        
        // Transfer to each beneficiary
        // Note: In a real implementation, you would use a token client for the actual transfers
        for i in 0..ben_addresses.len() {
            let beneficiary = ben_addresses.get(i).unwrap();
            let percentage = env.storage().persistent().get::<_, i128>(&DataKey::Beneficiary(beneficiary.clone())).unwrap_or(0);
            let amount = (total_deposited * percentage) / 100;
            
            assert!(amount > 0, "Transfer amount must be greater than 0");
            
            // This is where you would use a token client to transfer the tokens
            // For now, we just emit the event
            emit_beneficiary_paid(&env, &beneficiary, &amount);
        }
        
        // Reset total deposited since all funds are distributed
        env.storage().persistent().set(&DataKey::TotalDeposited, &0_i128);
    }
    
    // View functions
    pub fn get_owner(env: Env) -> Address {
        env.storage().persistent().get::<_, Address>(&DataKey::Owner).unwrap()
    }
    
    pub fn get_is_dead(env: Env) -> bool {
        env.storage().persistent().get::<_, bool>(&DataKey::IsDead).unwrap_or(false)
    }
    
    pub fn get_beneficiary_percentage(env: Env, address: Address) -> i128 {
        env.storage().persistent().get::<_, i128>(&DataKey::Beneficiary(address)).unwrap_or(0)
    }
    
    pub fn get_beneficiary_addresses(env: Env) -> Vec<Address> {
        env.storage().persistent().get::<_, Vec<Address>>(&DataKey::BeneficiaryAddresses).unwrap_or(Vec::new(&env))
    }
}

fn emit_beneficiary_added(env: &Env, beneficiary: &Address, percentage: &i128) {
    let topics = (Symbol::new(env, "beneficiary_added"), beneficiary);
    env.events().publish(topics, percentage);
}

fn emit_beneficiary_deleted(env: &Env, beneficiary: &Address) {
    let topics = (Symbol::new(env, "beneficiary_deleted"), beneficiary);
    env.events().publish(topics, &());
}

fn emit_core_deposited(env: &Env, sender: &Address, amount: &i128) {
    let topics = (Symbol::new(env, "core_deposited"), sender);
    env.events().publish(topics, amount);
}

fn emit_death_status_changed(env: &Env, is_dead: &bool) {
    let topics = (Symbol::new(env, "death_status_changed"),);
    env.events().publish(topics, is_dead);
}

fn emit_beneficiary_paid(env: &Env, beneficiary: &Address, amount: &i128) {
    let topics = (Symbol::new(env, "beneficiary_paid"), beneficiary);
    env.events().publish(topics, amount);
}

#[cfg(test)]
mod test;