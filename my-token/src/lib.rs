use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    AccountId,
};

#[derive(Default, BorshDeserialize, BorshSerialize)]
struct MyContract {
    data: u8,
}

impl ft_api::Contract for MyContract {
    fn transfer(&mut self, from: AccountId, to: AccountId, amount: u64) {
        near_sdk::log!("from: {}, to: {}, amount: {}", from, to, amount);
    }

    fn balance(&self, of: AccountId) -> u64 {
        near_sdk::log!("balance {}", of);
        8
    }
}

ft_api::register!(MyContract);

// #[no_mangle]
// extern "C" fn ft_get_vtable() -> Box<MyContract> {
//     Box::<MyContract>::new(near_sdk::env::state_read().unwrap_or_default())
// }
