#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol, Vec};

#[contract]
pub struct PayrollSystem;

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
#[derive(Clone,Copy, Debug, PartialEq)]
pub enum PayrollError {
    Unauthorized = 0,
    DuplicateEntry = 1,
    EmployeeNotFound = 2,
}

// Implement From<PayrollError> cho soroban_sdk::Error
impl From<PayrollError> for soroban_sdk::Error {
    fn from(e: PayrollError) -> soroban_sdk::Error {
        match e {
            PayrollError::Unauthorized => soroban_sdk::Error::from_contract_error(0),
            PayrollError::DuplicateEntry => soroban_sdk::Error::from_contract_error(1),
            PayrollError::EmployeeNotFound => soroban_sdk::Error::from_contract_error(2),
        }
    }
}

// THÊM: Implement From<&PayrollError> cho soroban_sdk::Error
impl From<&PayrollError> for soroban_sdk::Error {
    fn from(e: &PayrollError) -> soroban_sdk::Error {
        match e {
            PayrollError::Unauthorized => soroban_sdk::Error::from_contract_error(0),
            PayrollError::DuplicateEntry => soroban_sdk::Error::from_contract_error(1),
            PayrollError::EmployeeNotFound => soroban_sdk::Error::from_contract_error(2),
        }
    }
}

// Implement TryFrom<soroban_sdk::Error> cho PayrollError
impl TryFrom<soroban_sdk::Error> for PayrollError {
    type Error = soroban_sdk::Error;
    fn try_from(_err: soroban_sdk::Error) -> Result<Self, Self::Error> {
        Err(soroban_sdk::Error::from_contract_error(999))
    }
}

#[contractimpl]
impl PayrollSystem {
    pub fn initialize(env: Env, admin: Address) {
        env.storage().instance().set(&Symbol::new(&env, "ADMIN"), &admin);
    }

    pub fn add_employee(
        env: Env,
        admin: Address,
        employee_address: Address,
        name: String,
        salary: i128,
        role: String,
    ) -> Result<Employee, PayrollError> {
        admin.require_auth();

        let employees_key = Symbol::new(&env, "EMPLOYEES");
        let mut employees: Vec<Employee> = env
            .storage()
            .instance()
            .get(&employees_key)
            .unwrap_or(Vec::new(&env));

        // Kiểm tra trùng employee
        for i in 0..employees.len() {
            let existing_employee = employees.get(i).unwrap();
            if existing_employee.address == employee_address {
                return Err(PayrollError::DuplicateEntry);
            }
        }

        let employee = Employee {
            address: employee_address.clone(),
            name: name.clone(),
            salary,
            role: role.clone(),
            active: true,
        };

        employees.push_back(employee.clone());
        env.storage().instance().set(&employees_key, &employees);

        Ok(employee)
    }

    pub fn get_employees(env: Env) -> Vec<Employee> {
        let employees_key = Symbol::new(&env, "EMPLOYEES");
        env.storage()
            .instance()
            .get(&employees_key)
            .unwrap_or(Vec::new(&env))
    }

    pub fn update_employee_status(
        env: Env,
        admin: Address,
        employee_address: Address,
        active: bool,
    ) -> Result<Employee, PayrollError> {
        admin.require_auth();

        let employees_key = Symbol::new(&env, "EMPLOYEES");
        let mut employees: Vec<Employee> = env
            .storage()
            .instance()
            .get(&employees_key)
            .unwrap_or(Vec::new(&env));

        for i in 0..employees.len() {
            let mut employee = employees.get(i).unwrap();
            if employee.address == employee_address {
                employee.active = active;
                employees.set(i, employee.clone());
                env.storage().instance().set(&employees_key, &employees);
                return Ok(employee);
            }
        }

        Err(PayrollError::EmployeeNotFound)
    }

    pub fn hello(env: Env) -> String {
        String::from_str(&env, "Hello from PayrollSystem!")
    }
}