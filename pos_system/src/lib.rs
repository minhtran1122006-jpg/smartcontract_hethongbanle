#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, 
    Env, String, Address, Vec, Symbol, Map, Error
};

#[contract]
pub struct PosSystem;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Sale {
    pub sale_id: String,
    pub cashier: Address,
    pub customer: Option<Address>,
    pub items: Vec<SaleItem>,
    pub total_amount: i128,
    pub discount: i128,
    pub final_amount: i128,
    pub payment_method: PaymentMethod,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct SaleItem {
    pub sku: String,
    pub name: String,
    pub quantity: u32,
    pub unit_price: i128,
    pub total_price: i128,
}

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PaymentMethod {
    Cash,
    CreditCard,
    BankTransfer,
    Crypto,
    LoyaltyPoints,
}

#[contractimpl]
impl PosSystem {
    pub fn initialize(env: Env, admin: Address) {
        env.storage().instance().set(&Symbol::new(&env, "ADMIN"), &admin);
    }

    pub fn create_sale(
        env: Env,
        cashier: Address,
        customer: Option<Address>,
        items: Vec<SaleItem>,
        discount: i128,
        payment_method: PaymentMethod,
        _inventory_manager: Address,  // Thêm _ để tránh warning
        _payment_processor: Address,  // Thêm _ để tránh warning
    ) -> Result<Sale, Error> {
        cashier.require_auth();

        // Calculate totals
        let mut total_amount: i128 = 0;
        for i in 0..items.len() {
            let item = items.get(i).unwrap();
            total_amount += item.total_price;
            
            // Check inventory availability (mock)
            Self::_check_inventory_stock(env.clone(), _inventory_manager.clone(), item.sku.clone(), item.quantity)?;
        }

        let final_amount = if discount > 0 { total_amount - discount } else { total_amount };

        if final_amount <= 0 {
            return Err(Error::from_contract_error(1001)); // Invalid amount
        }

        // Process payment based on method (mock)
        match payment_method {
            PaymentMethod::Crypto => {
                Self::_process_crypto_payment(
                    env.clone(), 
                    customer.clone().unwrap_or(cashier.clone()), 
                    final_amount, 
                    _payment_processor
                )?;
            },
            PaymentMethod::LoyaltyPoints => {
                // Process loyalty points payment (mock)
            },
            _ => {
                // Cash, CreditCard, BankTransfer - record only
            }
        }

        // Update inventory (reduce stock) - mock
        Self::_update_inventory_after_sale(env.clone(), _inventory_manager.clone(), items.clone())?;

        // Tạo sale ID đơn giản không dùng format!
        let timestamp = env.ledger().timestamp();
        let sale_id = if timestamp % 2 == 0 {
            String::from_str(&env, "SALE_EVEN")
        } else {
            String::from_str(&env, "SALE_ODD")
        };

        let sale = Sale {
            sale_id,
            cashier: cashier.clone(),
            customer,
            items: items.clone(),
            total_amount,
            discount,
            final_amount,
            payment_method,
            timestamp,
        };

        // Store sale record
        let sales_key = Symbol::new(&env, "SALES");
        let mut sales: Vec<Sale> = env
            .storage()
            .instance()
            .get(&sales_key)
            .unwrap_or_else(|| Vec::new(&env));
        
        sales.push_back(sale.clone());
        env.storage().instance().set(&sales_key, &sales);

        // Emit sale event
        env.events().publish(
            (Symbol::new(&env, "sale_created"), cashier),
            sale.clone()
        );

        Ok(sale)
    }

    pub fn get_sale(env: Env, sale_id: String) -> Result<Sale, Error> {
        let sales: Vec<Sale> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "SALES"))
            .unwrap_or_else(|| Vec::new(&env));

        for i in 0..sales.len() {
            let sale = sales.get(i).unwrap();
            if sale.sale_id == sale_id {
                return Ok(sale.clone());
            }
        }

        Err(Error::from_contract_error(1002)) // Sale not found
    }

    pub fn get_sales_by_cashier(env: Env, cashier: Address) -> Vec<Sale> {
        let sales: Vec<Sale> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "SALES"))
            .unwrap_or_else(|| Vec::new(&env));

        let mut result = Vec::new(&env);
        for i in 0..sales.len() {
            let sale = sales.get(i).unwrap();
            if sale.cashier == cashier {
                result.push_back(sale.clone());
            }
        }
        result
    }

    pub fn get_sales_by_customer(env: Env, customer: Address) -> Vec<Sale> {
        let sales: Vec<Sale> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "SALES"))
            .unwrap_or_else(|| Vec::new(&env));

        let mut result = Vec::new(&env);
        for i in 0..sales.len() {
            let sale = sales.get(i).unwrap();
            if let Some(sale_customer) = &sale.customer {
                if sale_customer == &customer {
                    result.push_back(sale.clone());
                }
            }
        }
        result
    }

    pub fn get_daily_sales(env: Env, _date: String) -> Vec<Sale> {
        let sales: Vec<Sale> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "SALES"))
            .unwrap_or_else(|| Vec::new(&env));

        let mut result = Vec::new(&env);
        // Mock implementation - return all sales
        for i in 0..sales.len() {
            let sale = sales.get(i).unwrap();
            result.push_back(sale.clone());
        }
        result
    }

    // Helper functions với _ prefix để tránh warnings
    fn _check_inventory_stock(_env: Env, _inventory_manager: Address, _sku: String, _quantity: u32) -> Result<(), Error> {
        // Mock implementation
        Ok(())
    }

    fn _update_inventory_after_sale(_env: Env, _inventory_manager: Address, _items: Vec<SaleItem>) -> Result<(), Error> {
        // Mock implementation
        Ok(())
    }

    fn _process_crypto_payment(_env: Env, _customer: Address, _amount: i128, _payment_processor: Address) -> Result<(), Error> {
        // Mock implementation
        Ok(())
    }

    fn _timestamp_to_date(env: Env, _timestamp: u64) -> String {
        // Mock implementation
        String::from_str(&env, "2024-01-01")
    }

    pub fn get_total_sales_count(env: Env) -> u32 {
        let sales: Vec<Sale> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "SALES"))
            .unwrap_or_else(|| Vec::new(&env));
        
        sales.len() as u32
    }

    pub fn get_total_revenue(env: Env) -> i128 {
        let sales: Vec<Sale> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "SALES"))
            .unwrap_or_else(|| Vec::new(&env));

        let mut total: i128 = 0;
        for i in 0..sales.len() {
            let sale = sales.get(i).unwrap();
            total += sale.final_amount;
        }
        total
    }

    pub fn get_payment_method_stats(env: Env) -> Map<PaymentMethod, i128> {
        let sales: Vec<Sale> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "SALES"))
            .unwrap_or_else(|| Vec::new(&env));

        let mut stats = Map::new(&env);
        let mut cash_total = 0;
        let mut card_total = 0;
        let mut transfer_total = 0;
        let mut crypto_total = 0;
        let mut loyalty_total = 0;

        for i in 0..sales.len() {
            let sale = sales.get(i).unwrap();
            match sale.payment_method {
                PaymentMethod::Cash => cash_total += sale.final_amount,
                PaymentMethod::CreditCard => card_total += sale.final_amount,
                PaymentMethod::BankTransfer => transfer_total += sale.final_amount,
                PaymentMethod::Crypto => crypto_total += sale.final_amount,
                PaymentMethod::LoyaltyPoints => loyalty_total += sale.final_amount,
            }
        }

        stats.set(PaymentMethod::Cash, cash_total);
        stats.set(PaymentMethod::CreditCard, card_total);
        stats.set(PaymentMethod::BankTransfer, transfer_total);
        stats.set(PaymentMethod::Crypto, crypto_total);
        stats.set(PaymentMethod::LoyaltyPoints, loyalty_total);

        stats
    }

    pub fn hello(env: Env) -> String {
        String::from_str(&env, "Hello from POS System!")
    }
}