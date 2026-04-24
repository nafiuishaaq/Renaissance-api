use soroban_sdk::{contractimpl, Env, Symbol};

pub struct SafetyContract;

#[contractimpl]
impl SafetyContract {
    pub fn request_pause(env: Env) {
        let now = env.ledger().timestamp();
        env.storage().set(&Symbol::short("pause_request_time"), &now);
    }

    pub fn execute_pause(env: Env) {
        let now = env.ledger().timestamp();
        let request_time: u64 = env.storage().get(&Symbol::short("pause_request_time")).unwrap_or(0);

        if request_time > 0 && now >= request_time + 86400 {
            env.storage().set(&Symbol::short("paused"), &true);
        } else {
            panic!("Timelock not expired");
        }
    }

    pub fn unpause(env: Env) {
        env.storage().set(&Symbol::short("paused"), &false);
    }

    pub fn is_paused(env: Env) -> bool {
        env.storage().get(&Symbol::short("paused")).unwrap_or(false)
    }

    pub fn settle_position(env: Env, user: Address) {
        // Settlement logic allowed even if paused
        if Self::is_paused(env) {
            // still allow closing positions
        }
    }
}
