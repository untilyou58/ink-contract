#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod helloworld {
    use ink_storage::traits::SpreadAllocate;
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Helloworld {
        hello: ink_storage::Mapping<AccountId, u32>,
    }

    impl Helloworld {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(Self::new_init)
        }

        fn new_init(&mut self) {
            let caller = Self::env().caller();
            let value: u32 = Default::default();
            self.hello.insert(&caller, &value);
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|_| {})
        }

        #[ink(message)]
        pub fn add(&mut self, count: u32) {
            let caller = self.env().caller();
            let current_value = self.hello.get(&caller).unwrap_or_default();
            let new_value = count + current_value;
            self.hello.insert(&caller, &new_value);
        }

        #[ink(message)]
        pub fn get_value(&self, caller: AccountId) -> u32 {
            self.hello.get(&caller).unwrap_or_default()
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink_env::test;
        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        // fn set_caller(sender: AccountId) {
        //     ink_env::test::set_caller::<Environment>(sender);
        // }

        fn default_accounts() -> test::DefaultAccounts<Environment> {
            ink_env::test::default_accounts::<Environment>()
        }

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let helloworld = Helloworld::default();
            let accounts = default_accounts();
            assert_eq!(helloworld.get_value(accounts.alice), 0);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let accounts = default_accounts();
            let mut helloworld = Helloworld::new();
            assert_eq!(helloworld.get_value(accounts.alice), 0);
            helloworld.add(10);
            assert_eq!(helloworld.get_value(accounts.alice), 10);
        }
    }
}