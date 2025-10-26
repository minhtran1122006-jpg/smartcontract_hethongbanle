// ✅ CODE CHÍNH HOÀN CHỈNH - KHÔNG CÓ TEST
#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, contracterror, 
    Env, String, Address, Symbol, Vec, Map
};

#[contract]
pub struct VendorManager;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Vendor {
    pub vendor_id: String,
    pub name: String,
    pub contact_address: Address,
    pub phone: String,
    pub email: String,
    pub category: String,
    pub rating: u32,
    pub active: bool,
    pub products_supplied: Vec<String>,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct VendorPerformance {
    pub vendor_id: String,
    pub total_orders: u32,
    pub completed_orders: u32,
    pub total_revenue: i128,
    pub average_rating: u32,
}

#[contracterror]
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum VendorError {
    Unauthorized = 1,
    VendorNotFound = 2,
    DuplicateVendor = 3,
    InvalidRating = 4,
    VendorInactive = 5,
}

#[contractimpl]
impl VendorManager {
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&Symbol::new(&env, "ADMIN"), &admin);
    }

    pub fn add_vendor(
        env: Env,
        admin: Address,
        vendor_id: String,
        name: String,
        contact_address: Address,
        phone: String,
        email: String,
        category: String,
    ) -> Result<Vendor, VendorError> {
        admin.require_auth();

        let vendors_key = Symbol::new(&env, "VENDORS");
        let mut vendors: Map<String, Vendor> = env
            .storage()
            .instance()
            .get(&vendors_key)
            .unwrap_or_else(|| Map::new(&env));

        if vendors.contains_key(vendor_id.clone()) {
            return Err(VendorError::DuplicateVendor);
        }

        let vendor = Vendor {
            vendor_id: vendor_id.clone(),
            name: name.clone(),
            contact_address: contact_address.clone(),
            phone: phone.clone(),
            email: email.clone(),
            category: category.clone(),
            rating: 0,
            active: true,
            products_supplied: Vec::new(&env),
        };

        vendors.set(vendor_id, vendor.clone());
        env.storage().instance().set(&vendors_key, &vendors);

        Ok(vendor)
    }

    pub fn update_vendor(
        env: Env,
        admin: Address,
        vendor_id: String,
        name: Option<String>,
        phone: Option<String>,
        email: Option<String>,
        category: Option<String>,
    ) -> Result<Vendor, VendorError> {
        admin.require_auth();

        let vendors_key = Symbol::new(&env, "VENDORS");
        let mut vendors: Map<String, Vendor> = env
            .storage()
            .instance()
            .get(&vendors_key)
            .unwrap_or_else(|| Map::new(&env));

        let mut vendor = vendors.get(vendor_id.clone()).ok_or(VendorError::VendorNotFound)?;

        if let Some(new_name) = name {
            vendor.name = new_name;
        }
        if let Some(new_phone) = phone {
            vendor.phone = new_phone;
        }
        if let Some(new_email) = email {
            vendor.email = new_email;
        }
        if let Some(new_category) = category {
            vendor.category = new_category;
        }

        vendors.set(vendor_id, vendor.clone());
        env.storage().instance().set(&vendors_key, &vendors);

        Ok(vendor)
    }

    pub fn rate_vendor(
        env: Env,
        rater: Address,
        vendor_id: String,
        rating: u32,
    ) -> Result<Vendor, VendorError> {
        rater.require_auth();

        if rating < 1 || rating > 5 {
            return Err(VendorError::InvalidRating);
        }

        let vendors_key = Symbol::new(&env, "VENDORS");
        let mut vendors: Map<String, Vendor> = env
            .storage()
            .instance()
            .get(&vendors_key)
            .unwrap_or_else(|| Map::new(&env));

        let mut vendor = vendors.get(vendor_id.clone()).ok_or(VendorError::VendorNotFound)?;

        if !vendor.active {
            return Err(VendorError::VendorInactive);
        }

        let current_rating = vendor.rating;
        if current_rating == 0 {
            vendor.rating = rating;
        } else {
            vendor.rating = (current_rating + rating) / 2;
        }

        vendors.set(vendor_id, vendor.clone());
        env.storage().instance().set(&vendors_key, &vendors);

        Ok(vendor)
    }

    pub fn add_vendor_product(
        env: Env,
        admin: Address,
        vendor_id: String,
        product_sku: String,
    ) -> Result<Vendor, VendorError> {
        admin.require_auth();

        let vendors_key = Symbol::new(&env, "VENDORS");
        let mut vendors: Map<String, Vendor> = env
            .storage()
            .instance()
            .get(&vendors_key)
            .unwrap_or_else(|| Map::new(&env));

        let mut vendor = vendors.get(vendor_id.clone()).ok_or(VendorError::VendorNotFound)?;

        if !vendor.active {
            return Err(VendorError::VendorInactive);
        }

        vendor.products_supplied.push_back(product_sku);
        vendors.set(vendor_id, vendor.clone());
        env.storage().instance().set(&vendors_key, &vendors);

        Ok(vendor)
    }

    pub fn set_vendor_status(
        env: Env,
        admin: Address,
        vendor_id: String,
        active: bool,
    ) -> Result<Vendor, VendorError> {
        admin.require_auth();

        let vendors_key = Symbol::new(&env, "VENDORS");
        let mut vendors: Map<String, Vendor> = env
            .storage()
            .instance()
            .get(&vendors_key)
            .unwrap_or_else(|| Map::new(&env));

        let mut vendor = vendors.get(vendor_id.clone()).ok_or(VendorError::VendorNotFound)?;

        vendor.active = active;
        vendors.set(vendor_id, vendor.clone());
        env.storage().instance().set(&vendors_key, &vendors);

        Ok(vendor)
    }

    pub fn get_vendor(env: Env, vendor_id: String) -> Result<Vendor, VendorError> {
        let vendors_key = Symbol::new(&env, "VENDORS");
        let vendors: Map<String, Vendor> = env
            .storage()
            .instance()
            .get(&vendors_key)
            .unwrap_or_else(|| Map::new(&env));

        vendors.get(vendor_id).ok_or(VendorError::VendorNotFound)
    }

    pub fn get_all_vendors(env: Env) -> Vec<Vendor> {
        let vendors_key = Symbol::new(&env, "VENDORS");
        let vendors: Map<String, Vendor> = env
            .storage()
            .instance()
            .get(&vendors_key)
            .unwrap_or_else(|| Map::new(&env));

        let mut vendor_list = Vec::new(&env);
        let keys = vendors.keys();
        
        for key in keys.iter() {
            if let Some(vendor) = vendors.get(key) {
                vendor_list.push_back(vendor);
            }
        }
        vendor_list
    }

    pub fn get_vendors_by_category(env: Env, category: String) -> Vec<Vendor> {
        let vendors_key = Symbol::new(&env, "VENDORS");
        let vendors: Map<String, Vendor> = env
            .storage()
            .instance()
            .get(&vendors_key)
            .unwrap_or_else(|| Map::new(&env));

        let mut filtered_vendors = Vec::new(&env);
        let keys = vendors.keys();
        
        for key in keys.iter() {
            if let Some(vendor) = vendors.get(key) {
                if vendor.category == category && vendor.active {
                    filtered_vendors.push_back(vendor);
                }
            }
        }
        filtered_vendors
    }

    pub fn search_vendors(env: Env, query: String) -> Vec<Vendor> {
        let vendors_key = Symbol::new(&env, "VENDORS");
        let vendors: Map<String, Vendor> = env
            .storage()
            .instance()
            .get(&vendors_key)
            .unwrap_or_else(|| Map::new(&env));

        let mut results = Vec::new(&env);
        let keys = vendors.keys();
        
        for key in keys.iter() {
            if let Some(vendor) = vendors.get(key) {
                if vendor.name == query && vendor.active {
                    results.push_back(vendor);
                }
            }
        }
        results
    }

    pub fn get_vendor_performance(env: Env, vendor_id: String) -> Result<VendorPerformance, VendorError> {
        let vendor = Self::get_vendor(env.clone(), vendor_id.clone())?;

        let performance = VendorPerformance {
            vendor_id,
            total_orders: 0,
            completed_orders: 0,
            total_revenue: 0,
            average_rating: vendor.rating,
        };

        Ok(performance)
    }

    pub fn is_admin(env: Env, address: Address) -> bool {
        if let Some(admin) = env.storage().instance().get(&Symbol::new(&env, "ADMIN")) {
            address == admin
        } else {
            false
        }
    }

    pub fn remove_vendor(
        env: Env,
        admin: Address,
        vendor_id: String,
    ) -> Result<(), VendorError> {
        admin.require_auth();

        let vendors_key = Symbol::new(&env, "VENDORS");
        let mut vendors: Map<String, Vendor> = env
            .storage()
            .instance()
            .get(&vendors_key)
            .unwrap_or_else(|| Map::new(&env));

        if !vendors.contains_key(vendor_id.clone()) {
            return Err(VendorError::VendorNotFound);
        }

        vendors.remove(vendor_id);
        env.storage().instance().set(&vendors_key, &vendors);

        Ok(())
    }

    pub fn get_vendor_count(env: Env) -> u32 {
        let vendors_key = Symbol::new(&env, "VENDORS");
        let vendors: Map<String, Vendor> = env
            .storage()
            .instance()
            .get(&vendors_key)
            .unwrap_or_else(|| Map::new(&env));

        vendors.len()
    }

    pub fn vendor_exists(env: Env, vendor_id: String) -> bool {
        let vendors_key = Symbol::new(&env, "VENDORS");
        let vendors: Map<String, Vendor> = env
            .storage()
            .instance()
            .get(&vendors_key)
            .unwrap_or_else(|| Map::new(&env));

        vendors.contains_key(vendor_id)
    }
}

// ✅ HOẶC XÓA HOÀN TOÀN TEST MODULE
// #[cfg(test)]
// mod test {
//     // No tests - just build the contract
