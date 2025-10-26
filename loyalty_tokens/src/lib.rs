#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

#[contract]
pub struct LoyaltyToken;

#[contractimpl]
impl LoyaltyToken {
    pub fn init(env: Env, admin: Address) {
        // Yêu cầu người gọi phải là admin để khởi tạo
        admin.require_auth();
        let key = Symbol::new(&env, "admin");
        env.storage().persistent().set(&key, &admin);
    }

    pub fn issue_points(env: Env, to: Address, amount: i128) {
        let key = Symbol::new(&env, "admin");
        // Lấy admin từ persistent storage, bắt buộc phải có admin đã được set
        let admin: Address = env
            .storage()
            .persistent()
            .get::<Symbol, Address>(&key)
            .expect("Admin chưa được khởi tạo");
        // Yêu cầu admin phải xác thực
        admin.require_auth();

        // Lấy điểm hiện tại của người nhận, cộng thêm amount mới
        let mut balance = env.storage().persistent().get::<Address, i128>(&to).unwrap_or(0);
        balance += amount;
        env.storage().persistent().set(&to, &balance);
    }

    pub fn get_balance(env: Env, owner: Address) -> i128 {
        env.storage().persistent().get::<Address, i128>(&owner).unwrap_or(0)
    }

    pub fn redeem_points(env: Env, from: Address, amount: i128) {
        // Yêu cầu người dùng xác thực
        from.require_auth();

        let mut balance = env.storage().persistent().get::<Address, i128>(&from).unwrap_or(0);
        assert!(balance >= amount, "Not enough points");
        balance -= amount;
        env.storage().persistent().set(&from, &balance);
    }
}