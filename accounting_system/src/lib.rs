#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol, Map, Vec, Error};

#[contract]
pub struct AccountingSystem;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct FinancialTransaction {
    pub transaction_id: String,
    pub from_account: String,
    pub to_account: String,
    pub amount: i128,
    pub transaction_type: TransactionType,
    pub description: String,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TransactionType {
    Revenue,
    Expense,
    Transfer,
    Tax,
    Payroll,
    Investment,
    Refund,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct FinancialReport {
    pub period: String,
    pub total_revenue: i128,
    pub total_expenses: i128,
    pub net_income: i128,
    pub assets: i128,
    pub liabilities: i128,
    pub equity: i128,
    pub cash_flow: i128,
}

#[contractimpl]
impl AccountingSystem {
    pub fn initialize(env: Env, admin: Address) {
        env.storage().instance().set(&Symbol::new(&env, "ADMIN"), &admin);
    }

    pub fn record_transaction(
        env: Env,
        admin: Address,
        transaction: FinancialTransaction,
    ) -> Result<(), Error> {
        admin.require_auth();

        let transactions_key = Symbol::new(&env, "TRANSACTIONS");
        let mut transactions: Vec<FinancialTransaction> = env
            .storage()
            .instance()
            .get(&transactions_key)
            .unwrap_or_else(|| Vec::new(&env));

        transactions.push_back(transaction);
        env.storage().instance().set(&transactions_key, &transactions);

        Ok(())
    }

    pub fn generate_financial_report(
        env: Env,
        period: String,
    ) -> Result<FinancialReport, Error> {
        let transactions: Vec<FinancialTransaction> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "TRANSACTIONS"))
            .unwrap_or_else(|| Vec::new(&env));

        let mut total_revenue = 0;
        let mut total_expenses = 0;

        for i in 0..transactions.len() {
            let transaction = transactions.get(i).unwrap();
            match transaction.transaction_type {
                TransactionType::Revenue => total_revenue += transaction.amount,
                TransactionType::Expense | TransactionType::Tax | TransactionType::Payroll => {
                    total_expenses += transaction.amount
                },
                _ => {}
            }
        }

        let net_income = total_revenue - total_expenses;

        let report = FinancialReport {
            period,
            total_revenue,
            total_expenses,
            net_income,
            assets: total_revenue * 2, // Mock data
            liabilities: total_expenses, // Mock data
            equity: net_income * 3, // Mock data
            cash_flow: net_income - total_expenses / 2, // Mock data
        };

        Ok(report)
    }

    pub fn get_transaction_history(
        env: Env,
        account: String,
    ) -> Vec<FinancialTransaction> {
        let transactions: Vec<FinancialTransaction> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "TRANSACTIONS"))
            .unwrap_or_else(|| Vec::new(&env));

        let mut result = Vec::new(&env);
        for i in 0..transactions.len() {
            let transaction = transactions.get(i).unwrap();
            if transaction.from_account == account || transaction.to_account == account {
                result.push_back(transaction.clone());
            }
        }
        result
    }

    pub fn get_balance_sheet(env: Env) -> Result<Map<String, i128>, Error> {
        let mut balance_sheet = Map::new(&env);
        
        // Mock balance sheet data
        balance_sheet.set(String::from_str(&env, "cash"), 50_000_000);
        balance_sheet.set(String::from_str(&env, "accounts_receivable"), 25_000_000);
        balance_sheet.set(String::from_str(&env, "inventory"), 75_000_000);
        balance_sheet.set(String::from_str(&env, "total_assets"), 150_000_000);
        balance_sheet.set(String::from_str(&env, "accounts_payable"), 20_000_000);
        balance_sheet.set(String::from_str(&env, "loans"), 30_000_000);
        balance_sheet.set(String::from_str(&env, "total_liabilities"), 50_000_000);
        balance_sheet.set(String::from_str(&env, "equity"), 100_000_000);

        Ok(balance_sheet)
    }

    pub fn calculate_profit_margin(env: Env, period: String) -> Result<i128, Error> {
        let report = Self::generate_financial_report(env, period)?;
        
        if report.total_revenue == 0 {
            return Ok(0);
        }
        
        let profit_margin = (report.net_income * 100) / report.total_revenue;
        Ok(profit_margin)
    }

    pub fn hello(env: Env) -> String {
        String::from_str(&env, "Hello from Accounting System!")
    }
}