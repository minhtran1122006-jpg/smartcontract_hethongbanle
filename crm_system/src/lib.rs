#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, 
    Env, String, Address, Vec, Symbol, Map, Error
};

#[contract]
pub struct CrmSystem;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Customer {
    pub customer_id: String,
    pub wallet_address: Address,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub join_date: u64,
    pub total_spent: i128,
    pub total_orders: u32,
    pub loyalty_points: u32,
    pub customer_tier: CustomerTier,
    pub preferences: Vec<Preference>,
    pub last_purchase_date: u64,
    pub status: CustomerStatus,
}

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CustomerTier {
    Bronze,
    Silver,  
    Gold,
    Platinum,
}

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CustomerStatus {
    Active,
    Inactive,
    VIP,
    Blocked,
}

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Preference {
    EmailNotifications,
    SMSNotifications,
    ProductRecommendations,
    SpecialOffers,
    NewArrivalAlerts,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct CustomerUpdate {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub preferences: Option<Vec<Preference>>,
    pub status: Option<CustomerStatus>,
}

#[contractimpl]
impl CrmSystem {
    pub fn initialize(env: Env, admin: Address) {
        env.storage().instance().set(&Symbol::new(&env, "ADMIN"), &admin);
    }

    pub fn register_customer(
        env: Env,
        admin: Address,
        wallet_address: Address,
        name: String,
        email: String,
        phone: String,
    ) -> Result<Customer, Error> {
        admin.require_auth();

        let customers_key = Symbol::new(&env, "CUSTOMERS");
        let mut customers: Map<Address, Customer> = env
            .storage()
            .instance()
            .get(&customers_key)
            .unwrap_or_else(|| Map::new(&env));

        // Check for duplicate customer
        if customers.contains_key(wallet_address.clone()) {
            return Err(Error::from_contract_error(1001)); // Duplicate customer
        }

        // Tạo customer ID đơn giản
        let timestamp = env.ledger().timestamp();
        let customer_id = if timestamp % 2 == 0 {
            String::from_str(&env, "CUST_EVEN")
        } else {
            String::from_str(&env, "CUST_ODD")
        };

        let customer = Customer {
            customer_id,
            wallet_address: wallet_address.clone(),
            name,
            email,
            phone,
            join_date: timestamp,
            total_spent: 0,
            total_orders: 0,
            loyalty_points: 100,
            customer_tier: CustomerTier::Bronze,
            preferences: Vec::from_array(&env, [
                Preference::EmailNotifications,
                Preference::SpecialOffers,
            ]),
            last_purchase_date: 0,
            status: CustomerStatus::Active,
        };

        customers.set(wallet_address, customer.clone());
        env.storage().instance().set(&customers_key, &customers);

        Ok(customer)
    }

    pub fn update_customer(
        env: Env,
        customer_address: Address,
        updates: CustomerUpdate,
    ) -> Result<Customer, Error> {
        customer_address.require_auth();

        let customers_key = Symbol::new(&env, "CUSTOMERS");
        let mut customers: Map<Address, Customer> = env
            .storage()
            .instance()
            .get(&customers_key)
            .unwrap_or_else(|| Map::new(&env));

        let mut customer = customers.get(customer_address.clone())
            .ok_or(Error::from_contract_error(1002))?; // Customer not found

        // Update fields
        if let Some(name) = updates.name {
            customer.name = name;
        }
        if let Some(email) = updates.email {
            customer.email = email;
        }
        if let Some(phone) = updates.phone {
            customer.phone = phone;
        }
        if let Some(preferences) = updates.preferences {
            customer.preferences = preferences;
        }
        if let Some(status) = updates.status {
            customer.status = status;
        }

        customers.set(customer_address, customer.clone());
        env.storage().instance().set(&customers_key, &customers);

        Ok(customer)
    }

    pub fn record_purchase(
        env: Env,
        _pos_system: Address,
        customer_address: Address,
        amount: i128,
    ) -> Result<Customer, Error> {
        let customers_key = Symbol::new(&env, "CUSTOMERS");
        let mut customers: Map<Address, Customer> = env
            .storage()
            .instance()
            .get(&customers_key)
            .unwrap_or_else(|| Map::new(&env));

        let mut customer = customers.get(customer_address.clone())
            .ok_or(Error::from_contract_error(1002))?; // Customer not found

        // Update customer stats
        customer.total_spent += amount;
        customer.total_orders += 1;
        customer.last_purchase_date = env.ledger().timestamp();

        // Add loyalty points (1 point per 10,000 VND)
        let points_earned = amount / 10_000;
        customer.loyalty_points += points_earned as u32;

        // Update customer tier based on total spent
        customer.customer_tier = match customer.total_spent {
            t if t >= 20_000_000 => CustomerTier::Platinum,
            t if t >= 5_000_000 => CustomerTier::Gold,
            t if t >= 1_000_000 => CustomerTier::Silver,
            _ => CustomerTier::Bronze,
        };

        customers.set(customer_address, customer.clone());
        env.storage().instance().set(&customers_key, &customers);

        Ok(customer)
    }

    pub fn get_customer(env: Env, wallet_address: Address) -> Result<Customer, Error> {
        let customers: Map<Address, Customer> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "CUSTOMERS"))
            .unwrap_or_else(|| Map::new(&env));

        customers.get(wallet_address)
            .ok_or(Error::from_contract_error(1002)) // Customer not found
    }

    pub fn get_customers_by_tier(env: Env, tier: CustomerTier) -> Vec<Customer> {
        let customers: Map<Address, Customer> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "CUSTOMERS"))
            .unwrap_or_else(|| Map::new(&env));

        let mut result = Vec::new(&env);
        for customer in customers.values() {
            if customer.customer_tier == tier && customer.status == CustomerStatus::Active {
                result.push_back(customer);
            }
        }
        result
    }

    pub fn get_vip_customers(env: Env) -> Vec<Customer> {
        let customers: Map<Address, Customer> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "CUSTOMERS"))
            .unwrap_or_else(|| Map::new(&env));

        let mut result = Vec::new(&env);
        for customer in customers.values() {
            if customer.status == CustomerStatus::VIP {
                result.push_back(customer);
            }
        }
        result
    }

    pub fn get_customer_ranking(env: Env) -> Vec<Customer> {
        let customers: Map<Address, Customer> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "CUSTOMERS"))
            .unwrap_or_else(|| Map::new(&env));

        let customer_list: Vec<Customer> = customers.values();
        customer_list
    }

    pub fn add_loyalty_points(
        env: Env,
        admin: Address,
        customer_address: Address,
        points: u32,
    ) -> Result<Customer, Error> {
        admin.require_auth();

        let customers_key = Symbol::new(&env, "CUSTOMERS");
        let mut customers: Map<Address, Customer> = env
            .storage()
            .instance()
            .get(&customers_key)
            .unwrap_or_else(|| Map::new(&env));

        let mut customer = customers.get(customer_address.clone())
            .ok_or(Error::from_contract_error(1002))?; // Customer not found
        
        customer.loyalty_points += points;

        customers.set(customer_address, customer.clone());
        env.storage().instance().set(&customers_key, &customers);

        Ok(customer)
    }

    pub fn get_total_customers(env: Env) -> u32 {
        let customers: Map<Address, Customer> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "CUSTOMERS"))
            .unwrap_or_else(|| Map::new(&env));

        customers.len() as u32
    }

    pub fn get_customers_count_by_tier(env: Env) -> Map<CustomerTier, u32> {
        let customers: Map<Address, Customer> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "CUSTOMERS"))
            .unwrap_or_else(|| Map::new(&env));

        let mut count = Map::new(&env);
        let mut bronze = 0;
        let mut silver = 0;
        let mut gold = 0;
        let mut platinum = 0;

        for customer in customers.values() {
            match customer.customer_tier {
                CustomerTier::Bronze => bronze += 1,
                CustomerTier::Silver => silver += 1,
                CustomerTier::Gold => gold += 1,
                CustomerTier::Platinum => platinum += 1,
            }
        }

        count.set(CustomerTier::Bronze, bronze);
        count.set(CustomerTier::Silver, silver);
        count.set(CustomerTier::Gold, gold);
        count.set(CustomerTier::Platinum, platinum);

        count
    }

    pub fn hello(env: Env) -> String {
        String::from_str(&env, "Hello from CRM System!")
    }
}