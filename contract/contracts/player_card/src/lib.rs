#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec, U256, Symbol};

mod errors;
mod events;
mod storage;
mod token;

pub use errors::*;
pub use events::*;
pub use storage::*;
pub use token::*;

use common::{NFTMintEvent, NFT_MINT_EVENT, create_nft_mint_event};

#[contract]
pub struct PlayerCardContract;

#[contractimpl]
impl PlayerCardContract {
    /// Initialize the contract with the given admin
    pub fn initialize(env: Env, admin: Address) {
        if storage::has_admin(&env) {
            panic!("already initialized");
        }
        
        storage::set_admin(&env, &admin);
        storage::set_next_token_id(&env, 1);
        
        let event = NFTMintEvent {
            token_id: U256::from_u32(&env, 0),
            to: admin.clone(),
            token_uri: String::from_str(&env, "contract_initialized"),
            nft_contract: env.current_contract_address(),
            timestamp: env.ledger().timestamp(),
            mint_type: Symbol::short("INIT"),
            metadata: soroban_sdk::Map::new(&env),
            price: None,
        };
        
        env.events().publish((NFT_MINT_EVENT,), event);
    }

    /// Mint a new player card NFT to the specified recipient
    pub fn mint(env: Env, to: Address, token_uri: String) -> u64 {
        let admin = storage::get_admin(&env);
        admin.require_auth();

        let token_id = storage::get_next_token_id(&env);
        storage::increment_next_token_id(&env);

        storage::set_owner(&env, token_id, &to);
        storage::set_token_uri(&env, token_id, &token_uri);

        let event = create_nft_mint_event(
            &env,
            U256::from_u32(&env, token_id as u32),
            to.clone(),
            token_uri.clone(),
            env.current_contract_address(),
            Symbol::short("PLAYER_CARD"),
            None,
        );
        
        let mut event_with_timestamp = event;
        event_with_timestamp.timestamp = env.ledger().timestamp();

        env.events().publish((NFT_MINT_EVENT,), event_with_timestamp);

        token_id
    }

    /// Transfer ownership of a token from one address to another
    pub fn transfer(env: Env, from: Address, to: Address, token_id: u64) {
        from.require_auth();
        
        let current_owner = storage::get_owner(&env, token_id);
        if current_owner != from {
            panic!("not token owner");
        }

        storage::set_owner(&env, token_id, &to);

        let event = NFTMintEvent {
            token_id: U256::from_u32(&env, token_id as u32),
            to: to.clone(),
            token_uri: storage::get_token_uri(&env, token_id),
            nft_contract: env.current_contract_address(),
            timestamp: env.ledger().timestamp(),
            mint_type: Symbol::short("TRANSFER"),
            metadata: soroban_sdk::Map::new(&env),
            price: None,
        };

        env.events().publish((NFT_MINT_EVENT,), event);
    }

    /// Get the owner of a specific token
    pub fn owner_of(env: Env, token_id: u64) -> Address {
        storage::get_owner(&env, token_id)
    }

    /// Get the metadata URI for a specific token
    pub fn token_uri(env: Env, token_id: u64) -> String {
        storage::get_token_uri(&env, token_id)
    }

    /// Get total number of tokens minted
    pub fn total_supply(env: Env) -> u64 {
        storage::get_next_token_id(&env) - 1
    }

    /// Get all tokens owned by a specific address
    pub fn tokens_of_owner(env: Env, owner: Address) -> Vec<u64> {
        storage::get_tokens_of_owner(&env, owner)
    }
}
