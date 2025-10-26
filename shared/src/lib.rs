#![no_std]
use soroban_sdk::{contracttype, Address, String};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Product {
    pub sku: String,
    pub name: String,
    pub price: i128,
    pub stock_quantity: u32,
    pub supplier: Address,
    pub location: String,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Employee {
    pub address: Address,
    pub name: String,
    pub salary: i128,
    pub role: String,
    pub active: bool,
}

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RetailError {
    Unauthorized = 0,
    InsufficientBalance = 1,
    ProductNotFound = 2,
    InsufficientStock = 3,
    InvalidAmount = 4,
    DuplicateEntry = 5,
    EmployeeNotFound = 6,
}

// Utility functions
pub fn calculate_loyalty_points(amount: i128) -> i128 {
    amount / 1000  // 1 point per 1000 units
}

// CHỈ CẦN Implement From<RetailError> cho soroban_sdk::Error
// Không cần TryFrom ngược lại
impl From<RetailError> for soroban_sdk::Error {
    fn from(e: RetailError) -> soroban_sdk::Error {
        match e {
            RetailError::Unauthorized => soroban_sdk::Error::from_contract_error(0),
            RetailError::InsufficientBalance => soroban_sdk::Error::from_contract_error(1),
            RetailError::ProductNotFound => soroban_sdk::Error::from_contract_error(2),
            RetailError::InsufficientStock => soroban_sdk::Error::from_contract_error(3),
            RetailError::InvalidAmount => soroban_sdk::Error::from_contract_error(4),
            RetailError::DuplicateEntry => soroban_sdk::Error::from_contract_error(5),
            RetailError::EmployeeNotFound => soroban_sdk::Error::from_contract_error(6),
        }
    }
}