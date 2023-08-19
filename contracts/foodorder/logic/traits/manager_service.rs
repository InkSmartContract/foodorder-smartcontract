use crate::impls::types::{CourierResult, ManagerResult, RestaurantResult};
use ink::prelude::string::String;
use openbrush::traits::AccountId;

/// Manager Service Definition
#[openbrush::trait_definition]
pub trait ManagerService {
    /// Function that service manager adds a restaurant
    #[ink(message)]
    fn add_restaurant(
        &mut self,
        restaurant_account: AccountId,
        restaurant_name: String,
        restaurant_address: String,
        phone_number: String,
    ) -> RestaurantResult;

    /// Function that service manager adds a courier
    #[ink(message)]
    fn add_courier(
        &mut self,
        courier_account: AccountId,
        courier_name: String,
        courier_address: String,
        phone_number: String,
    ) -> CourierResult;

    /// Function that service manager changes account
    #[ink(message)]
    fn change_manager(&mut self, new_account: AccountId) -> ManagerResult;

    /// Function that service manager changes courier service surcharge rate
    #[ink(message)]
    fn change_fee_rate(&mut self, rate: u32) -> ManagerResult;
}
