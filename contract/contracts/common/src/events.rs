use soroban_sdk::{contracttype, Address, Symbol, BytesN};

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

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpinExecutedEvent {
    pub spin_id: BytesN<32>,
    pub executor: Address,
    pub timestamp: u64,
}
