#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol, Vec, Map, Error};

#[contract]
pub struct XMoneyToken;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct XMoneyPayment {
    pub payment_id: String,
    pub from: Address,
    pub to: Address,
    pub amount: i128,
    pub currency: Currency,
    pub transaction_hash: String,
    pub status: PaymentStatus,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Currency {
    XMT,
    USDC,
    VNDT,
}

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
    Refunded,
}

#[contractimpl]
impl XMoneyToken {
    pub fn initialize(env: Env, admin: Address, total_supply: i128) {
        env.storage().instance().set(&Symbol::new(&env, "ADMIN"), &admin);
        env.storage().instance().set(&Symbol::new(&env, "TOTAL_SUPPLY"), &total_supply);
        
        let mut balances: Map<Address, i128> = Map::new(&env);
        balances.set(admin.clone(), total_supply);
        env.storage().instance().set(&Symbol::new(&env, "BALANCES"), &balances);
    }

    pub fn balance_of(env: Env, address: Address) -> i128 {
        let balances: Map<Address, i128> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "BALANCES"))
            .unwrap_or_else(|| Map::new(&env));
        
        balances.get(address).unwrap_or(0)
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) -> Result<(), Error> {
        from.require_auth();
        
        if amount <= 0 {
            return Err(Error::from_contract_error(3001));
        }

        let mut balances: Map<Address, i128> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "BALANCES"))
            .unwrap_or_else(|| Map::new(&env));

        let from_balance = balances.get(from.clone()).unwrap_or(0);
        if from_balance < amount {
            return Err(Error::from_contract_error(3002));
        }

        balances.set(from.clone(), from_balance - amount);
        let to_balance = balances.get(to.clone()).unwrap_or(0);
        balances.set(to.clone(), to_balance + amount);

        env.storage().instance().set(&Symbol::new(&env, "BALANCES"), &balances);

        Ok(())
    }

    pub fn pay_with_xmoney(
        env: Env,
        customer: Address,
        merchant: Address,
        amount: i128,
        order_id: String,
    ) -> Result<XMoneyPayment, Error> {
        customer.require_auth();

        Self::transfer(env.clone(), customer.clone(), merchant.clone(), amount)?;

        let timestamp = env.ledger().timestamp();
        
        // Táº¡o payment ID Ä‘Æ¡n giáº£n khÃ´ng dÃ¹ng concat
        let payment_id = order_id; // Dùng order_id từ POS system làm payment_id

        let transaction_hash = String::from_str(&env, "XM_TX_HASH");

        let payment = XMoneyPayment {
            payment_id,
            from: customer.clone(),
            to: merchant.clone(),
            amount,
            currency: Currency::XMT,
            transaction_hash,
            status: PaymentStatus::Completed,
            timestamp,
        };

        let payments_key = Symbol::new(&env, "PAYMENTS");
        let mut payments: Vec<XMoneyPayment> = env
            .storage()
            .instance()
            .get(&payments_key)
            .unwrap_or_else(|| Vec::new(&env));
        
        payments.push_back(payment.clone());
        env.storage().instance().set(&payments_key, &payments);

        Ok(payment)
    }

    pub fn mint(env: Env, admin: Address, to: Address, amount: i128) -> Result<(), Error> {
        admin.require_auth();

        let admin_addr: Address = env.storage().instance().get(&Symbol::new(&env, "ADMIN"))
            .ok_or(Error::from_contract_error(3003))?;

        if admin != admin_addr {
            return Err(Error::from_contract_error(3004));
        }

        if amount <= 0 {
            return Err(Error::from_contract_error(3001));
        }

        let mut balances: Map<Address, i128> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "BALANCES"))
            .unwrap_or_else(|| Map::new(&env));

        let to_balance = balances.get(to.clone()).unwrap_or(0);
        balances.set(to.clone(), to_balance + amount);

        let total_supply: i128 = env.storage().instance().get(&Symbol::new(&env, "TOTAL_SUPPLY"))
            .ok_or(Error::from_contract_error(3005))?;

        env.storage().instance().set(&Symbol::new(&env, "TOTAL_SUPPLY"), &(total_supply + amount));
        env.storage().instance().set(&Symbol::new(&env, "BALANCES"), &balances);

        Ok(())
    }

    pub fn get_total_supply(env: Env) -> Result<i128, Error> {
        env.storage().instance().get(&Symbol::new(&env, "TOTAL_SUPPLY"))
            .ok_or(Error::from_contract_error(3005))
    }

    pub fn get_payment_history(env: Env, address: Address) -> Vec<XMoneyPayment> {
        let payments: Vec<XMoneyPayment> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "PAYMENTS"))
            .unwrap_or_else(|| Vec::new(&env));

        let mut result = Vec::new(&env);
        for i in 0..payments.len() {
            let payment = payments.get(i).unwrap();
            if payment.from == address || payment.to == address {
                result.push_back(payment.clone());
            }
        }
        result
    }

    pub fn get_merchant_payments(env: Env, merchant: Address) -> Vec<XMoneyPayment> {
        let payments: Vec<XMoneyPayment> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "PAYMENTS"))
            .unwrap_or_else(|| Vec::new(&env));

        let mut result = Vec::new(&env);
        for i in 0..payments.len() {
            let payment = payments.get(i).unwrap();
            if payment.to == merchant && payment.status == PaymentStatus::Completed {
                result.push_back(payment.clone());
            }
        }
        result
    }

    pub fn get_customer_payments(env: Env, customer: Address) -> Vec<XMoneyPayment> {
        let payments: Vec<XMoneyPayment> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "PAYMENTS"))
            .unwrap_or_else(|| Vec::new(&env));

        let mut result = Vec::new(&env);
        for i in 0..payments.len() {
            let payment = payments.get(i).unwrap();
            if payment.from == customer {
                result.push_back(payment.clone());
            }
        }
        result
    }

    pub fn hello(env: Env) -> String {
        String::from_str(&env, "Hello from XMoney Token!")
    }
}
