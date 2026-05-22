#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short,
    Address, Env, String, Symbol, Vec,
};

// ============================================================
// CONSTANTS
// ============================================================

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_TTL: u32 = 7 * DAY_IN_LEDGERS;
const INSTANCE_THRESHOLD: u32 = 6 * DAY_IN_LEDGERS;
const PERSISTENT_TTL: u32 = 30 * DAY_IN_LEDGERS;
const PERSISTENT_THRESHOLD: u32 = 29 * DAY_IN_LEDGERS;

// ============================================================
// DATA TYPES
// ============================================================

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CourseInfo {
    pub course_id: String,
    pub course_name: String,
    pub class_id: String,
    pub time_slot: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SwapStatus {
    Active,
    Completed,
    Canceled,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapRequest {
    pub id: u32,
    pub offered_course: CourseInfo,
    pub wanted_course: CourseInfo,
    pub owner: Address,
    pub status: SwapStatus,
}

/// Dữ liệu trả về cho public (Ẩn danh tính owner)
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnonymousSwapRequest {
    pub id: u32,
    pub offered_course: CourseInfo,
    pub wanted_course: CourseInfo,
}

#[contracttype]
pub enum DataKey {
    Admin,
    RequestCounter,
    Request(u32),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    NotAuthorized = 1,
    NotFound = 2,
    InvalidStatus = 3,
    MismatchCourse = 4,
    AlreadyInitialized = 5,
}

// ============================================================
// CONTRACT
// ============================================================

#[contract]
pub struct CourseSwapContract;

#[contractimpl]
impl CourseSwapContract {
    /// Khởi tạo Admin
    pub fn init(env: Env, admin: Address) -> Result<(), ContractError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(ContractError::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::RequestCounter, &0u32);
        env.storage().instance().extend_ttl(INSTANCE_THRESHOLD, INSTANCE_TTL);
        Ok(())
    }

    /// Đăng tin đổi môn (Public)
    pub fn post_request(
        env: Env,
        user: Address,
        offered: CourseInfo,
        wanted: CourseInfo,
    ) -> Result<u32, ContractError> {
        user.require_auth();

        let mut counter: u32 = env.storage().instance().get(&DataKey::RequestCounter).unwrap_or(0);
        counter += 1;

        let request = SwapRequest {
            id: counter,
            offered_course: offered,
            wanted_course: wanted,
            owner: user.clone(),
            status: SwapStatus::Active,
        };

        let key = DataKey::Request(counter);
        env.storage().persistent().set(&key, &request);
        env.storage().persistent().extend_ttl(&key, PERSISTENT_THRESHOLD, PERSISTENT_TTL);

        env.storage().instance().set(&DataKey::RequestCounter, &counter);

        // Emit event (ẩn danh owner trong topic nếu cần, ở đây ta chỉ báo có ID mới)
        env.events().publish((symbol_short!("posted"), counter), ());

        Ok(counter)
    }

    /// Xem các tin đang active (Ẩn danh)
    pub fn get_available_swaps(env: Env) -> Vec<AnonymousSwapRequest> {
        let counter: u32 = env.storage().instance().get(&DataKey::RequestCounter).unwrap_or(0);
        let mut results = Vec::new(&env);

        for i in 1..=counter {
            if let Some(req) = env.storage().persistent().get::<DataKey, SwapRequest>(&DataKey::Request(i)) {
                if req.status == SwapStatus::Active {
                    results.push_back(AnonymousSwapRequest {
                        id: req.id,
                        offered_course: req.offered_course,
                        wanted_course: req.wanted_course,
                    });
                }
            }
        }
        results
    }

    /// Thực hiện đổi môn (Atomic)
    pub fn execute_swap(
        env: Env,
        user: Address,
        request_id: u32,
        user_course: CourseInfo,
    ) -> Result<(), ContractError> {
        user.require_auth();

        let key = DataKey::Request(request_id);
        let mut req: SwapRequest = env.storage().persistent().get(&key).ok_or(ContractError::NotFound)?;

        if req.status != SwapStatus::Active {
            return Err(ContractError::InvalidStatus);
        }

        // KIỂM TRA TÍNH KHỚP LỆNH: Môn user đang có phải là môn chủ request muốn
        if user_course != req.wanted_course {
            return Err(ContractError::MismatchCourse);
        }

        // Cập nhật trạng thái
        req.status = SwapStatus::Completed;
        env.storage().persistent().set(&key, &req);

        // Thông báo giao dịch thành công (vẫn ẩn danh người thực hiện trong public event nếu cần)
        env.events().publish((symbol_short!("swapped"), request_id), ());

        Ok(())
    }

    /// Admin hủy tin rác
    pub fn admin_cancel(env: Env, admin: Address, request_id: u32) -> Result<(), ContractError> {
        admin.require_auth();
        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).ok_or(ContractError::NotAuthorized)?;
        if admin != stored_admin {
            return Err(ContractError::NotAuthorized);
        }

        let key = DataKey::Request(request_id);
        let mut req: SwapRequest = env.storage().persistent().get(&key).ok_or(ContractError::NotFound)?;

        req.status = SwapStatus::Canceled;
        env.storage().persistent().set(&key, &req);

        env.events().publish((symbol_short!("canceled"), request_id), ());
        Ok(())
    }
}

mod test;
