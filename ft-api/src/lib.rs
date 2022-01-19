use near_sdk::AccountId;

pub trait Contract: Send + Sync {
    fn transfer(&mut self, from: AccountId, to: AccountId, amount: u64);
    fn balance(&self, of: AccountId) -> u64;
}

// Blockchain-side API *and* implementation of the contract. Physically, "API"
// is some `(func (export "name"))` in WASM, which is represented as `extern
// "C"` in Rust.
//
// Note that this is an actual **implementation** of the said API -- these are
// functions declared & defined in the upstream crate, which use "dynamic"
// dispatch internally to call downstream code
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

    unsafe { ft_get_vtable() }.transfer(from, to, amount)
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

    let result = unsafe { ft_get_vtable() }.balance(of);

    let result = near_sdk::serde_json::to_vec(&result)
        .expect("Failed to serialize the return value using JSON.");
    near_sdk::env::value_return(&result);
}

// This is the hook which is defined upstream, but whose implementation is going
// to be defined downstream.
extern "C" {
    #[allow(improper_ctypes)]
    fn ft_get_vtable() -> Box<dyn Contract>;
}

// This allows the user to register their contract as the implementation for the
// singleton instance.
//
// The salient point here is that method signatures don't have to be specified,
// they are encoded solely by the trait.
#[macro_export]
macro_rules! register {
    ($e:ty) => {
        #[no_mangle]
        extern "C" fn ft_get_vtable() -> Box<$e> {
            Box::<$e>::new(near_sdk::env::state_read().unwrap_or_default())
        }
    };
}
