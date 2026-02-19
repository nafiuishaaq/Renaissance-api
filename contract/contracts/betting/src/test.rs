#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::*, Env, Address, BytesN, Symbol};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register(&BettingContract, ());
    let backend_signer = Address::generate(&env);

    BettingContractClient::new(&env, &contract_id)
        .initialize(&backend_signer);

    // Verify initialization by checking stored signer
    let storage = env.storage().persistent();
    let stored_signer: Address = storage
        .get(&Symbol::new(&env, "backend_signer"))
        .unwrap();

    assert_eq!(stored_signer, backend_signer);
}

#[test]
fn test_spin_execution_success() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(&BettingContract, ());
    let backend_signer = Address::generate(&env);
    let executor = Address::generate(&env);

    let client = BettingContractClient::new(&env, &contract_id);
    
    // Initialize contract
    client.initialize(&backend_signer);

    // Create spin execution data
    let spin_id: BytesN<32> = BytesN::from_array(&env, &[1u8; 32]);
    let spin_hash: BytesN<32> = BytesN::from_array(&env, &[2u8; 32]);
    let signature: BytesN<64> = BytesN::from_array(&env, &[3u8; 64]);

    // Execute spin
    let result = client.execute_spin(&spin_id, &spin_hash, &signature, &executor);
    
    // Should succeed
    assert!(result.is_ok());
}

#[test]
fn test_prevent_duplicate_spin_execution() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(&BettingContract, ());
    let backend_signer = Address::generate(&env);
    let executor = Address::generate(&env);

    let client = BettingContractClient::new(&env, &contract_id);
    
    client.initialize(&backend_signer);

    let spin_id: BytesN<32> = BytesN::from_array(&env, &[1u8; 32]);
    let spin_hash: BytesN<32> = BytesN::from_array(&env, &[2u8; 32]);
    let signature: BytesN<64> = BytesN::from_array(&env, &[3u8; 64]);

    // First execution should succeed
    let result1 = client.execute_spin(&spin_id, &spin_hash, &signature, &executor);
    assert!(result1.is_ok());

    // Second execution with same spin_id should fail
    let result2 = client.execute_spin(&spin_id, &spin_hash, &signature, &executor);
    assert!(result2.is_err());
}

#[test]
fn test_prevent_replay_attacks() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(&BettingContract, ());
    let backend_signer = Address::generate(&env);
    let executor = Address::generate(&env);

    let client = BettingContractClient::new(&env, &contract_id);
    
    client.initialize(&backend_signer);

    let spin_id_1: BytesN<32> = BytesN::from_array(&env, &[1u8; 32]);
    let spin_id_2: BytesN<32> = BytesN::from_array(&env, &[4u8; 32]);
    let spin_hash: BytesN<32> = BytesN::from_array(&env, &[2u8; 32]);
    let signature: BytesN<64> = BytesN::from_array(&env, &[3u8; 64]);

    // First execution with spin_hash
    let result1 = client.execute_spin(&spin_id_1, &spin_hash, &signature, &executor);
    assert!(result1.is_ok());

    // Second execution with same spin_hash but different spin_id should fail
    let result2 = client.execute_spin(&spin_id_2, &spin_hash, &signature, &executor);
    assert!(result2.is_err());
}

#[test]
fn test_is_spin_executed() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(&BettingContract, ());
    let backend_signer = Address::generate(&env);
    let executor = Address::generate(&env);

    let client = BettingContractClient::new(&env, &contract_id);
    
    client.initialize(&backend_signer);

    let spin_id: BytesN<32> = BytesN::from_array(&env, &[1u8; 32]);
    let spin_hash: BytesN<32> = BytesN::from_array(&env, &[2u8; 32]);
    let signature: BytesN<64> = BytesN::from_array(&env, &[3u8; 64]);

    // Before execution
    assert!(!client.is_spin_executed(&spin_id));

    // Execute spin
    client.execute_spin(&spin_id, &spin_hash, &signature, &executor).unwrap();

    // After execution
    assert!(client.is_spin_executed(&spin_id));
}

#[test]
fn test_get_spin_execution() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(&BettingContract, ());
    let backend_signer = Address::generate(&env);
    let executor = Address::generate(&env);

    let client = BettingContractClient::new(&env, &contract_id);
    
    client.initialize(&backend_signer);

    let spin_id: BytesN<32> = BytesN::from_array(&env, &[1u8; 32]);
    let spin_hash: BytesN<32> = BytesN::from_array(&env, &[2u8; 32]);
    let signature: BytesN<64> = BytesN::from_array(&env, &[3u8; 64]);

    client.execute_spin(&spin_id, &spin_hash, &signature, &executor).unwrap();

    let execution = client.get_spin_execution(&spin_id).unwrap();
    
    assert_eq!(execution.spin_id, spin_id);
    assert_eq!(execution.executor, executor);
}

#[test]
fn test_is_spin_hash_used() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(&BettingContract, ());
    let backend_signer = Address::generate(&env);
    let executor = Address::generate(&env);

    let client = BettingContractClient::new(&env, &contract_id);
    
    client.initialize(&backend_signer);

    let spin_id: BytesN<32> = BytesN::from_array(&env, &[1u8; 32]);
    let spin_hash: BytesN<32> = BytesN::from_array(&env, &[2u8; 32]);
    let signature: BytesN<64> = BytesN::from_array(&env, &[3u8; 64]);

    // Before execution
    assert!(!client.is_spin_hash_used(&spin_hash));

    // Execute spin
    client.execute_spin(&spin_id, &spin_hash, &signature, &executor).unwrap();

    // After execution
    assert!(client.is_spin_hash_used(&spin_hash));
}
