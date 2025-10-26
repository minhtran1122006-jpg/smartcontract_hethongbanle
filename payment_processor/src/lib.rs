#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, String, Symbol, Vec, token,
};

#[contract]
pub struct PaymentProcessor;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct PaymentRecord {
    pub payment_id: String,
    pub customer: Address,
    pub merchant: Address,
    pub amount: i128,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PaymentError {
    Unauthorized = 0,
    InvalidAmount = 1,
    PaymentNotFound = 2,
}

impl From<PaymentError> for soroban_sdk::Error {
    fn from(e: PaymentError) -> soroban_sdk::Error {
        match e {
            PaymentError::Unauthorized => soroban_sdk::Error::from_contract_error(0),
            PaymentError::InvalidAmount => soroban_sdk::Error::from_contract_error(1),
            PaymentError::PaymentNotFound => soroban_sdk::Error::from_contract_error(2),
        }
    }
}

impl From<&PaymentError> for soroban_sdk::Error {
    fn from(e: &PaymentError) -> soroban_sdk::Error {
        match e {
            PaymentError::Unauthorized => soroban_sdk::Error::from_contract_error(0),
            PaymentError::InvalidAmount => soroban_sdk::Error::from_contract_error(1),
            PaymentError::PaymentNotFound => soroban_sdk::Error::from_contract_error(2),
        }
    }
}

impl TryFrom<soroban_sdk::Error> for PaymentError {
    type Error = soroban_sdk::Error;
    fn try_from(_err: soroban_sdk::Error) -> Result<Self, Self::Error> {
        Err(soroban_sdk::Error::from_contract_error(999))
    }
}

#[contractimpl]
impl PaymentProcessor {
    pub fn initialize(env: Env, admin: Address) {
        env.storage().instance().set(&Symbol::new(&env, "ADMIN"), &admin);
    }

    pub fn process_payment(
        env: Env,
        customer: Address,
        merchant: Address,
        amount: i128,
        token: Address,
    ) -> Result<PaymentRecord, PaymentError> {
        
        customer.require_auth();

        if amount <= 0 {
            return Err(PaymentError::InvalidAmount);
        }

        // Transfer tokens
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&customer, &merchant, &amount);

        // Create payment record - ĐƠN GIẢN HÓA payment_id
        let timestamp = env.ledger().timestamp();
    
        let payment_id = String::from_str(&env, "PAYMENT");

// Kết quả: payment_id = "1678901234" (timestamp)

        let record = PaymentRecord {
            payment_id,
            customer: customer.clone(),
            merchant: merchant.clone(),
            amount,
            timestamp,
        };

        // Store payment record
        let payments_key = Symbol::new(&env, "PAYMENTS");
        let mut payments: Vec<PaymentRecord> = env
            .storage()
            .instance()
            .get(&payments_key)
            .unwrap_or(Vec::new(&env));
        
        payments.push_back(record.clone());
        env.storage().instance().set(&payments_key, &payments);

        Ok(record)
    }

    pub fn get_payment_history(env: Env, customer: Address) -> Vec<PaymentRecord> {
        let payments_key = Symbol::new(&env, "PAYMENTS");
        let all_payments: Vec<PaymentRecord> = env
            .storage()
            .instance()
            .get(&payments_key)
            .unwrap_or(Vec::new(&env));
        
        let mut customer_payments = Vec::new(&env);
        
        for i in 0..all_payments.len() {
            let payment = all_payments.get(i).unwrap();
            if payment.customer == customer {
                customer_payments.push_back(payment.clone());
            }
        }
        
        customer_payments
    }

    pub fn get_all_payments(env: Env, admin: Address) -> Result<Vec<PaymentRecord>, PaymentError> {
        admin.require_auth();
        
        let payments_key = Symbol::new(&env, "PAYMENTS");
        let payments: Vec<PaymentRecord> = env
            .storage()
            .instance()
            .get(&payments_key)
            .unwrap_or(Vec::new(&env));
            
        Ok(payments)
    }

    pub fn hello(env: Env) -> String {
        String::from_str(&env, "Hello from PaymentProcessor!")
    }
} 