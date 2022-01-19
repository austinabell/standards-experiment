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
}

ft_api::register!(MyContract);
