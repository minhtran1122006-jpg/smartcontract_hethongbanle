#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, 
    Env, String, Address, Vec, Symbol, Map
};

#[contract]
pub struct EmployeeManager;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Employee {
    pub employee_id: String,
    pub wallet_address: Address,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub department: Department,
    pub position: String,
    pub hire_date: u64,
    pub status: EmployeeStatus,
    pub role: EmployeeRole,
    pub salary_grade: u32,
    pub permissions: Vec<Permission>,
}

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EmployeeStatus {
    Active,
    OnLeave,
    Terminated,
    Suspended,
}

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EmployeeRole {
    SuperAdmin,
    StoreManager,
    DepartmentManager,
    Cashier,
    StockClerk,
    SalesAssociate,
    HRManager,
    Accountant,
}

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Department {
    Executive,
    Sales,
    Inventory,
    Finance,
    HumanResources,
    CustomerService,
    IT,
    Marketing,
}

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Permission {
    ManageEmployees,
    ProcessPayments,
    ManageInventory,
    ViewReports,
    ManageVendors,
    IssueLoyaltyPoints,
    ProcessPayroll,
    SystemAdmin,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct EmployeeUpdate {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub department: Option<Department>,
    pub position: Option<String>,
    pub status: Option<EmployeeStatus>,
    pub role: Option<EmployeeRole>,
    pub salary_grade: Option<u32>,
}

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EmployeeError {
    Unauthorized,
    EmployeeNotFound,
    DuplicateEmployee,
    InvalidDepartment,
    InsufficientPermissions,
    EmployeeTerminated,
}

impl From<EmployeeError> for soroban_sdk::Error {
    fn from(e: EmployeeError) -> Self {
        match e {
            EmployeeError::Unauthorized => soroban_sdk::Error::from_contract_error(0),
            EmployeeError::EmployeeNotFound => soroban_sdk::Error::from_contract_error(1),
            EmployeeError::DuplicateEmployee => soroban_sdk::Error::from_contract_error(2),
            EmployeeError::InvalidDepartment => soroban_sdk::Error::from_contract_error(3),
            EmployeeError::InsufficientPermissions => soroban_sdk::Error::from_contract_error(4),
            EmployeeError::EmployeeTerminated => soroban_sdk::Error::from_contract_error(5),
        }
    }
}

impl From<&EmployeeError> for soroban_sdk::Error {
    fn from(e: &EmployeeError) -> Self {
        match e {
            EmployeeError::Unauthorized => soroban_sdk::Error::from_contract_error(0),
            EmployeeError::EmployeeNotFound => soroban_sdk::Error::from_contract_error(1),
            EmployeeError::DuplicateEmployee => soroban_sdk::Error::from_contract_error(2),
            EmployeeError::InvalidDepartment => soroban_sdk::Error::from_contract_error(3),
            EmployeeError::InsufficientPermissions => soroban_sdk::Error::from_contract_error(4),
            EmployeeError::EmployeeTerminated => soroban_sdk::Error::from_contract_error(5),
        }
    }
}

impl TryFrom<soroban_sdk::Error> for EmployeeError {
    type Error = soroban_sdk::Error;
    
    fn try_from(_err: soroban_sdk::Error) -> Result<Self, Self::Error> {
        Err(soroban_sdk::Error::from_contract_error(999))
    }
}

#[contractimpl]
impl EmployeeManager {
    pub fn initialize(env: Env, super_admin: Address) {
        env.storage().instance().set(&Symbol::new(&env, "SUPER_ADMIN"), &super_admin);
        
        // Initialize default permissions for roles
        let mut role_permissions = Map::new(&env);
        
        // SuperAdmin has all permissions
        role_permissions.set(
            EmployeeRole::SuperAdmin,
            Vec::from_array(&env, [
                Permission::ManageEmployees,
                Permission::ProcessPayments,
                Permission::ManageInventory,
                Permission::ViewReports,
                Permission::ManageVendors,
                Permission::IssueLoyaltyPoints,
                Permission::ProcessPayroll,
                Permission::SystemAdmin,
            ])
        );
        
        // StoreManager permissions
        role_permissions.set(
            EmployeeRole::StoreManager,
            Vec::from_array(&env, [
                Permission::ManageEmployees,
                Permission::ProcessPayments,
                Permission::ManageInventory,
                Permission::ViewReports,
                Permission::ManageVendors,
            ])
        );
        
        // Cashier permissions
        role_permissions.set(
            EmployeeRole::Cashier,
            Vec::from_array(&env, [
                Permission::ProcessPayments,
                Permission::IssueLoyaltyPoints,
            ])
        );
        
        // StockClerk permissions
        role_permissions.set(
            EmployeeRole::StockClerk,
            Vec::from_array(&env, [
                Permission::ManageInventory,
            ])
        );
        
        env.storage().instance().set(&Symbol::new(&env, "ROLE_PERMISSIONS"), &role_permissions);
    }

    pub fn add_employee(
        env: Env,
        admin: Address,
        employee_id: String,
        wallet_address: Address,
        name: String,
        email: String,
        phone: String,
        department: Department,
        position: String,
        role: EmployeeRole,
        salary_grade: u32,
    ) -> Result<Employee, EmployeeError> {
        admin.require_auth();
        
        // Check if caller has permission to manage employees
        Self::check_permission(env.clone(), admin.clone(), Permission::ManageEmployees)?;
        let employees_key = Symbol::new(&env, "EMPLOYEES");
        let mut employees: Map<String, Employee> = env
            .storage()
            .instance()
            .get(&employees_key)
            .unwrap_or(Map::new(&env));

        // Check for duplicate employee ID
        if employees.contains_key(employee_id.clone()) {
            return Err(EmployeeError::DuplicateEmployee);
        }

        // Get default permissions for the role
        let role_permissions: Map<EmployeeRole, Vec<Permission>> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "ROLE_PERMISSIONS"))
            .unwrap_or(Map::new(&env));

        let permissions = role_permissions.get(role).unwrap_or(Vec::new(&env));

        let employee = Employee {
            employee_id: employee_id.clone(),
            wallet_address,
            name,
            email,
            phone,
            department,
            position,
            hire_date: env.ledger().timestamp(),
            status: EmployeeStatus::Active,
            role,
            salary_grade,
            permissions,
        };

        employees.set(employee_id, employee.clone());
        env.storage().instance().set(&employees_key, &employees);

        Ok(employee)
    }

    pub fn update_employee(
        env: Env,
        admin: Address,
        employee_id: String,
        updates: EmployeeUpdate,
    ) -> Result<Employee, EmployeeError> {
        admin.require_auth();
        Self::check_permission(env.clone(), admin.clone(), Permission::ManageEmployees)?;

        let employees_key = Symbol::new(&env, "EMPLOYEES");
        let mut employees: Map<String, Employee> = env
            .storage()
            .instance()
            .get(&employees_key)
            .unwrap_or(Map::new(&env));

        let mut employee = employees.get(employee_id.clone()).ok_or(EmployeeError::EmployeeNotFound)?;

        // Update fields if provided
        if let Some(name) = updates.name {
            employee.name = name;
        }
        if let Some(email) = updates.email {
            employee.email = email;
        }
        if let Some(phone) = updates.phone {
            employee.phone = phone;
        }
        if let Some(department) = updates.department {
            employee.department = department;
        }
        if let Some(position) = updates.position {
            employee.position = position;
        }
        if let Some(status) = updates.status {
            employee.status = status;
        }
        if let Some(role) = updates.role {
            employee.role = role;
            // Update permissions when role changes
            let role_permissions: Map<EmployeeRole, Vec<Permission>> = env
                .storage()
                .instance()
                .get(&Symbol::new(&env, "ROLE_PERMISSIONS"))
                .unwrap_or(Map::new(&env));
            employee.permissions = role_permissions.get(role).unwrap_or(Vec::new(&env));
        }
        if let Some(salary_grade) = updates.salary_grade {
            employee.salary_grade = salary_grade;
        }

        employees.set(employee_id, employee.clone());
        env.storage().instance().set(&employees_key, &employees);

        Ok(employee)
    }

    pub fn terminate_employee(
        env: Env,
        admin: Address,
        employee_id: String,
    ) -> Result<(), EmployeeError> {
        admin.require_auth();
        Self::check_permission(env.clone(), admin.clone(), Permission::ManageEmployees)?;

        let employees_key = Symbol::new(&env, "EMPLOYEES");
        let mut employees: Map<String, Employee> = env
            .storage()
            .instance()
            .get(&employees_key)
            .unwrap_or(Map::new(&env));

        let mut employee = employees.get(employee_id.clone()).ok_or(EmployeeError::EmployeeNotFound)?;
        
        employee.status = EmployeeStatus::Terminated;
        employees.set(employee_id, employee);
        env.storage().instance().set(&employees_key, &employees);

        Ok(())
    }

    pub fn get_employee(env: Env, employee_id: String) -> Result<Employee, EmployeeError> {
        let employees: Map<String, Employee> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "EMPLOYEES"))
            .unwrap_or(Map::new(&env));

        employees.get(employee_id).ok_or(EmployeeError::EmployeeNotFound)
    }

    pub fn get_employee_by_wallet(env: Env, wallet_address: Address) -> Result<Employee, EmployeeError> {
        let employees: Map<String, Employee> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "EMPLOYEES"))
            .unwrap_or(Map::new(&env));

        for employee in employees.values() {
            if employee.wallet_address == wallet_address {
                return Ok(employee);
            }
        }

        Err(EmployeeError::EmployeeNotFound)
    }

    pub fn get_employees_by_department(env: Env, department: Department) -> Vec<Employee> {
        let employees: Map<String, Employee> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "EMPLOYEES"))
            .unwrap_or(Map::new(&env));

        let mut result = Vec::new(&env);
        
        for employee in employees.values() {
            if employee.department == department && employee.status == EmployeeStatus::Active {
                result.push_back(employee);
            }
        }

        result
    }

    pub fn check_permission(env: Env, employee_address: Address, required_permission: Permission) -> Result<(), EmployeeError> {
        let employee = Self::get_employee_by_wallet(env.clone(), employee_address.clone())?;
        
        if employee.status != EmployeeStatus::Active {
            return Err(EmployeeError::EmployeeTerminated);
        }

        for permission in employee.permissions.iter() {
            if permission == required_permission {
                return Ok(());
            }
        }

        Err(EmployeeError::InsufficientPermissions)
    }

    pub fn has_permission(env: Env, employee_address: Address, required_permission: Permission) -> bool {
        Self::check_permission(env.clone(), employee_address.clone(), required_permission).is_ok()
    }

    pub fn get_all_employees(env: Env) -> Vec<Employee> {
        let employees: Map<String, Employee> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "EMPLOYEES"))
            .unwrap_or(Map::new(&env));

        employees.values()
    }

    pub fn get_active_employees_count(env: Env) -> u32 {
        let employees: Map<String, Employee> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "EMPLOYEES"))
            .unwrap_or(Map::new(&env));

        let mut count = 0;
        for employee in employees.values() {
            if employee.status == EmployeeStatus::Active {
                count += 1;
            }
        }
        count
    }

    pub fn hello(env: Env) -> String {
        String::from_str(&env, "Hello from EmployeeManager!")
    }
}