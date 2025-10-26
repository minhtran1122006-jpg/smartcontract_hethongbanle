#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, 
    Env, String, Address, Vec, Symbol, Map, Error
};

#[contract]
pub struct Analytics;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct DailyReport {
    pub date: String,
    pub total_sales: i128,
    pub total_transactions: u32,
    pub average_transaction: i128,
    pub cash_sales: i128,
    pub card_sales: i128,
    pub crypto_sales: i128,
    pub top_products: Vec<ProductSales>,
    pub new_customers: u32,
    pub inventory_turnover: i128,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct ProductSales {
    pub sku: String,
    pub name: String,
    pub quantity_sold: u32,
    pub revenue: i128,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct FinancialReport {
    pub period: String,
    pub total_revenue: i128,
    pub total_expenses: i128,
    pub net_profit: i128,
    pub gross_margin: i128,
    pub best_performing_products: Vec<ProductSales>,
    pub customer_acquisition_cost: i128,
    pub customer_lifetime_value: i128,
}

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Period {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
}

#[contractimpl]
impl Analytics {
    pub fn initialize(env: Env, admin: Address) {
        env.storage().instance().set(&Symbol::new(&env, "ADMIN"), &admin);
    }

    pub fn generate_daily_report(
        env: Env,
        admin: Address,
        date: String,
        _pos_system: Address,
        _crm_system: Address,
    ) -> Result<DailyReport, Error> {
        admin.require_auth();

        // Mock data
        let report = DailyReport {
            date: date.clone(),
            total_sales: 15_870_000,
            total_transactions: 47,
            average_transaction: 337_659,
            cash_sales: 8_450_000,
            card_sales: 5_120_000,
            crypto_sales: 2_300_000,
            top_products: Vec::from_array(&env, [
                ProductSales {
                    sku: String::from_str(&env, "PROD_001"),
                    name: String::from_str(&env, "iPhone 15 Pro"),
                    quantity_sold: 3,
                    revenue: 45_000_000,
                },
                ProductSales {
                    sku: String::from_str(&env, "PROD_002"),
                    name: String::from_str(&env, "Samsung Galaxy S24"),
                    quantity_sold: 2,
                    revenue: 28_000_000,
                },
            ]),
            new_customers: 12,
            inventory_turnover: 240,
        };

        // Store report
        let reports_key = Symbol::new(&env, "DAILY_REPORTS");
        let mut reports: Map<String, DailyReport> = env
            .storage()
            .instance()
            .get(&reports_key)
            .unwrap_or_else(|| Map::new(&env));

        reports.set(date, report.clone());
        env.storage().instance().set(&reports_key, &reports);

        Ok(report)
    }

    pub fn get_daily_report(env: Env, date: String) -> Result<DailyReport, Error> {
        let reports: Map<String, DailyReport> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "DAILY_REPORTS"))
            .unwrap_or_else(|| Map::new(&env));

        reports.get(date).ok_or(Error::from_contract_error(1001)) // SỬA Ở ĐÂY
    }

    pub fn generate_financial_report(
        env: Env,
        admin: Address,
        period: Period,
        _start_date: String,
        _end_date: String,
    ) -> Result<FinancialReport, Error> {
        admin.require_auth();

        let report = FinancialReport {
            period: match period {
                Period::Daily => String::from_str(&env, "Daily"),
                Period::Weekly => String::from_str(&env, "Weekly"),
                Period::Monthly => String::from_str(&env, "Monthly"),
                Period::Quarterly => String::from_str(&env, "Quarterly"),
                Period::Yearly => String::from_str(&env, "Yearly"),
            },
            total_revenue: 485_000_000,
            total_expenses: 320_000_000,
            net_profit: 165_000_000,
            gross_margin: 3402,
            best_performing_products: Vec::from_array(&env, [
                ProductSales {
                    sku: String::from_str(&env, "PROD_001"),
                    name: String::from_str(&env, "iPhone 15 Pro"),
                    quantity_sold: 25,
                    revenue: 375_000_000,
                },
                ProductSales {
                    sku: String::from_str(&env, "PROD_003"),
                    name: String::from_str(&env, "MacBook Air M2"),
                    quantity_sold: 8,
                    revenue: 192_000_000,
                },
            ]),
            customer_acquisition_cost: 850_000,
            customer_lifetime_value: 4_500_000,
        };

        Ok(report)
    }

    pub fn get_sales_trend(
        env: Env,
        _days: u32,
    ) -> Vec<i128> {
        Vec::from_array(&env, [
            12_500_000, 15_870_000, 18_240_000, 14_560_000, 
            16_980_000, 20_150_000, 22_430_000,
        ])
    }

    pub fn get_top_customers(
        env: Env,
        _limit: u32,
    ) -> Vec<(Address, i128)> {
        Vec::from_array(&env, [
            (
                Address::from_string(&String::from_str(&env, "GABCD123456789")),
                45_000_000
            ),
            (
                Address::from_string(&String::from_str(&env, "GDEFG123456789")),
                38_500_000
            ),
            (
                Address::from_string(&String::from_str(&env, "GHIJK123456789")),
                32_000_000
            ),
        ])
    }

    pub fn get_inventory_analytics(
        env: Env,
    ) -> Map<String, u32> {
        let mut analytics = Map::new(&env);
        analytics.set(String::from_str(&env, "total_products"), 156);
        analytics.set(String::from_str(&env, "low_stock_items"), 12);
        analytics.set(String::from_str(&env, "out_of_stock_items"), 3);
        analytics.set(String::from_str(&env, "slow_moving_items"), 23);
        
        analytics
    }

    pub fn get_kpis(env: Env) -> Map<String, i128> {
        let mut kpis = Map::new(&env);
        kpis.set(String::from_str(&env, "daily_revenue"), 15_870_000);
        kpis.set(String::from_str(&env, "monthly_revenue"), 485_000_000);
        kpis.set(String::from_str(&env, "customer_count"), 1247);
        kpis.set(String::from_str(&env, "average_order_value"), 337_659);
        kpis.set(String::from_str(&env, "conversion_rate"), 68);
        
        kpis
    }

    pub fn hello(env: Env) -> String {
        String::from_str(&env, "Hello from Analytics System!")
    }
}