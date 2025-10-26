#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol, Map, Error};

#[contract]
pub struct Dashboard;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct RealTimeMetrics {
    pub total_sales_today: i128,
    pub total_customers_today: u32,
    pub total_orders_today: u32,
    pub revenue_today: i128,
    pub average_order_value: i128,
    pub low_stock_alerts: u32,
    pub pending_orders: u32,
    pub online_visitors: u32,
}

#[contractimpl]
impl Dashboard {
    pub fn initialize(env: Env, admin: Address) {
        env.storage().instance().set(&Symbol::new(&env, "ADMIN"), &admin);
    }

    pub fn get_real_time_metrics(env: Env) -> Result<RealTimeMetrics, Error> {
        let metrics = RealTimeMetrics {
            total_sales_today: 47,
            total_customers_today: 124,
            total_orders_today: 89,
            revenue_today: 15_870_000,
            average_order_value: 337_659,
            low_stock_alerts: 12,
            pending_orders: 8,
            online_visitors: 23,
        };

        Ok(metrics)
    }

    pub fn hello(env: Env) -> String {
        String::from_str(&env, "Hello from Dashboard System!")
    }
}