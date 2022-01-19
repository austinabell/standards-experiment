use near_sdk::AccountId;

pub trait Contract: Send + Sync {
    fn transfer(&mut self, from: AccountId, to: AccountId, amount: u64);
    fn balance(&self, of: AccountId) -> u64 {
        // Testing default
        near_sdk::log!("balance {}", of);
        8
    }
}

// This allows the user to register their contract as the implementation for the
// singleton instance.
//
// The salient point here is that method signatures don't have to be specified,
// they are encoded solely by the trait.
#[macro_export]
macro_rules! register {
    ($e:ty) => {
        fn load_state() -> $e {
            near_sdk::env::state_read().unwrap_or_default()
        }

        pub extern "C" fn transfer() {
            #[derive(near_sdk::serde::Deserialize)]
            #[serde(crate = "near_sdk::serde")]
            struct Input {
                from: AccountId,
                to: AccountId,
                amount: u64,
            }
            let Input { from, to, amount }: Input = near_sdk::serde_json::from_slice(
                &near_sdk::env::input().expect("Expected input since method has arguments."),
            )
            .expect("Failed to deserialize input from JSON.");

            let mut contract = load_state();
            $crate::Contract::transfer(&mut contract, from, to, amount);
            near_sdk::env::state_write(&contract);

        }

        pub extern "C" fn balance() {
            #[derive(near_sdk::serde::Deserialize)]
            #[serde(crate = "near_sdk::serde")]
            struct Input {
                of: AccountId,
            }
            let Input { of }: Input = near_sdk::serde_json::from_slice(
                &near_sdk::env::input().expect("Expected input since method has arguments."),
            )
            .expect("Failed to deserialize input from JSON.");

            let result = $crate::Contract::balance(&load_state(), of);

            let result = near_sdk::serde_json::to_vec(&result)
                .expect("Failed to serialize the return value using JSON.");
            near_sdk::env::value_return(&result);
        }
    };
}
