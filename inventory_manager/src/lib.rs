#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Symbol, String, Address, Vec, 
};

#[contract]
pub struct InventoryManager;

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
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RetailError {
    Unauthorized,
    ProductNotFound,
    DuplicateEntry,
    InsufficientStock,
}

// ✅ Chuyển từ RetailError sang soroban_sdk::Error
impl From<RetailError> for soroban_sdk::Error {
    fn from(e: RetailError) -> Self {
        match e {
            RetailError::Unauthorized => soroban_sdk::Error::from_contract_error(0),
            RetailError::ProductNotFound => soroban_sdk::Error::from_contract_error(1),
            RetailError::DuplicateEntry => soroban_sdk::Error::from_contract_error(2),
            RetailError::InsufficientStock => soroban_sdk::Error::from_contract_error(3),
        }
    }
}

// ✅ Chuyển từ &RetailError sang soroban_sdk::Error
impl From<&RetailError> for soroban_sdk::Error {
    fn from(e: &RetailError) -> Self {
        match e {
            RetailError::Unauthorized => soroban_sdk::Error::from_contract_error(0),
            RetailError::ProductNotFound => soroban_sdk::Error::from_contract_error(1),
            RetailError::DuplicateEntry => soroban_sdk::Error::from_contract_error(2),
            RetailError::InsufficientStock => soroban_sdk::Error::from_contract_error(3),
        }
    }
}

// ✅ Chuyển từ soroban_sdk::Error sang RetailError (cho trait bound)
impl TryFrom<soroban_sdk::Error> for RetailError {
    type Error = soroban_sdk::Error;
    
    fn try_from(err: soroban_sdk::Error) -> Result<Self, Self::Error> {
        // Map specific error codes to RetailError
        match err {
            _ => Err(err), // Hoặc custom mapping nếu cần
        }
    }
}

#[contractimpl]
impl InventoryManager {
    pub fn initialize(env: Env, admin: Address) {
        env.storage().instance().set(&Symbol::new(&env, "ADMIN"), &admin);
    }

    pub fn add_product(
        env: Env,
        admin: Address,
        sku: String,
        name: String,
        price: i128,
        initial_stock: u32,
        supplier: Address,
        location: String,
    ) -> Result<Product, RetailError> {
        admin.require_auth();

        let products_key = Symbol::new(&env, "PRODUCTS");
        let mut products: Vec<Product> = env
            .storage()
            .instance()
            .get(&products_key)
            .unwrap_or(Vec::new(&env));

        // Kiểm tra trùng SKU
        for i in 0..products.len() {
            let existing_product = products.get(i).unwrap();
            if existing_product.sku == sku {
                return Err(RetailError::DuplicateEntry);
            }
        }

        // Clone supplier trước khi move
        let supplier_clone = supplier.clone();

        let product = Product {
            sku: sku.clone(),
            name: name.clone(),
            price,
            stock_quantity: initial_stock,
            supplier,
            location: location.clone(),
        };

        products.push_back(product.clone());
        env.storage().instance().set(&products_key, &products);

        // ✅ EVENT: Product Added - dùng supplier_clone
        env.events().publish(
            (Symbol::new(&env, "product_added"),),
            (sku.clone(), name.clone(), price, initial_stock, supplier_clone, location)
        );

        Ok(product)
    }

    pub fn update_stock(
        env: Env,
        admin: Address,
        sku: String,
        new_quantity: u32,
    ) -> Result<Product, RetailError> {
        admin.require_auth();

        let products_key = Symbol::new(&env, "PRODUCTS");
        let mut products: Vec<Product> = env
            .storage()
            .instance()
            .get(&products_key)
            .unwrap_or(Vec::new(&env));

        let mut found_index = None;

        for i in 0..products.len() {
            let product = products.get(i).unwrap();
            if product.sku == sku {
                found_index = Some(i);
                break;
            }
        }

        if let Some(index) = found_index {
            let old_product = products.get(index).unwrap();
            let updated_product = Product {
                sku: old_product.sku.clone(),
                name: old_product.name.clone(),
                price: old_product.price,
                stock_quantity: new_quantity,
                supplier: old_product.supplier.clone(),
                location: old_product.location.clone(),
            };

            products.set(index, updated_product.clone());
            env.storage().instance().set(&products_key, &products);

            // ✅ EVENT: Stock Updated
            env.events().publish(
                (Symbol::new(&env, "stock_updated"),),
                (sku.clone(), new_quantity)
            );

            Ok(updated_product)
        } else {
            Err(RetailError::ProductNotFound)
        }
    }

    pub fn get_product(env: Env, sku: String) -> Result<Product, RetailError> {
        let products_key = Symbol::new(&env, "PRODUCTS");
        let products: Vec<Product> = env
            .storage()
            .instance()
            .get(&products_key)
            .unwrap_or(Vec::new(&env));

        for i in 0..products.len() {
            let product = products.get(i).unwrap();
            if product.sku == sku {
                return Ok(product.clone());
            }
        }

        Err(RetailError::ProductNotFound)
    }

    pub fn get_all_products(env: Env) -> Vec<Product> {
        env.storage()
            .instance()
            .get(&Symbol::new(&env, "PRODUCTS"))
            .unwrap_or(Vec::new(&env))
    }
}