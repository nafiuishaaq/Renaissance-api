use soroban_sdk::{contracttype, Address, Symbol, String, U256, Map, Env};

// ===== CORE EVENTS =====

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StakeEvent {
    pub user: Address,
    pub amount: i128,
    pub token_address: Address,
    pub staking_contract: Address,
    pub timestamp: u64,
    pub stake_id: U256,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnstakeEvent {
    pub user: Address,
    pub amount: i128,
    pub token_address: Address,
    pub staking_contract: Address,
    pub timestamp: u64,
    pub stake_id: U256,
    pub rewards: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BetEvent {
    pub bettor: Address,
    pub amount: i128,
    pub bet_id: U256,
    pub betting_contract: Address,
    pub timestamp: u64,
    pub bet_type: Symbol,
    pub odds: u32,
    pub metadata: Map<Symbol, String>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SettlementEvent {
    pub bet_id: U256,
    pub winner: Address,
    pub payout: i128,
    pub betting_contract: Address,
    pub timestamp: u64,
    pub settlement_type: Symbol,
    pub final_odds: u32,
    pub metadata: Map<Symbol, String>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpinRewardEvent {
    pub user: Address,
    pub reward_amount: i128,
    pub token_address: Address,
    pub game_contract: Address,
    pub timestamp: u64,
    pub spin_id: U256,
    pub reward_type: Symbol,
    pub multiplier: u32,
    pub metadata: Map<Symbol, String>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NFTMintEvent {
    pub token_id: U256,
    pub to: Address,
    pub token_uri: String,
    pub nft_contract: Address,
    pub timestamp: u64,
    pub mint_type: Symbol,
    pub metadata: Map<Symbol, String>,
    pub price: Option<i128>,
}

// ===== LEGACY EVENTS (for backward compatibility) =====

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BetPlacedEvent {
    pub bettor: Address,
    pub bet_id: Symbol,
    pub amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BetSettledEvent {
    pub bet_id: Symbol,
    pub winner: Address,
    pub payout: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BetCancelledEvent {
    pub bet_id: Symbol,
    pub reason: Symbol,
}

// ===== EVENT CONSTANTS =====

pub const STAKE_EVENT: Symbol = Symbol::short("STAKE");
pub const UNSTAKE_EVENT: Symbol = Symbol::short("UNSTAKE");
pub const BET_EVENT: Symbol = Symbol::short("BET");
pub const SETTLEMENT_EVENT: Symbol = Symbol::short("SETTLE");
pub const SPIN_REWARD_EVENT: Symbol = Symbol::short("SPIN_RWD");
pub const NFT_MINT_EVENT: Symbol = Symbol::short("NFT_MINT");

// ===== EVENT HELPERS =====

pub fn create_stake_event(
    user: Address,
    amount: i128,
    token_address: Address,
    staking_contract: Address,
    stake_id: U256,
) -> StakeEvent {
    StakeEvent {
        user,
        amount,
        token_address,
        staking_contract,
        timestamp: 0, // Will be set by contract
        stake_id,
    }
}

pub fn create_unstake_event(
    user: Address,
    amount: i128,
    token_address: Address,
    staking_contract: Address,
    stake_id: U256,
    rewards: i128,
) -> UnstakeEvent {
    UnstakeEvent {
        user,
        amount,
        token_address,
        staking_contract,
        timestamp: 0, // Will be set by contract
        stake_id,
        rewards,
    }
}

pub fn create_bet_event(
    env: &Env,
    bettor: Address,
    amount: i128,
    bet_id: U256,
    betting_contract: Address,
    bet_type: Symbol,
    odds: u32,
) -> BetEvent {
    BetEvent {
        bettor,
        amount,
        bet_id,
        betting_contract,
        timestamp: 0, // Will be set by contract
        bet_type,
        odds,
        metadata: Map::new(env),
    }
}

pub fn create_settlement_event(
    env: &Env,
    bet_id: U256,
    winner: Address,
    payout: i128,
    betting_contract: Address,
    settlement_type: Symbol,
    final_odds: u32,
) -> SettlementEvent {
    SettlementEvent {
        bet_id,
        winner,
        payout,
        betting_contract,
        timestamp: 0, // Will be set by contract
        settlement_type,
        final_odds,
        metadata: Map::new(env),
    }
}

pub fn create_spin_reward_event(
    env: &Env,
    user: Address,
    reward_amount: i128,
    token_address: Address,
    game_contract: Address,
    spin_id: U256,
    reward_type: Symbol,
    multiplier: u32,
) -> SpinRewardEvent {
    SpinRewardEvent {
        user,
        reward_amount,
        token_address,
        game_contract,
        timestamp: 0, // Will be set by contract
        spin_id,
        reward_type,
        multiplier,
        metadata: Map::new(env),
    }
}

pub fn create_nft_mint_event(
    env: &Env,
    token_id: U256,
    to: Address,
    token_uri: String,
    nft_contract: Address,
    mint_type: Symbol,
    price: Option<i128>,
) -> NFTMintEvent {
    NFTMintEvent {
        token_id,
        to,
        token_uri,
        nft_contract,
        timestamp: 0, // Will be set by contract
        mint_type,
        metadata: Map::new(env),
        price,
    }
}
