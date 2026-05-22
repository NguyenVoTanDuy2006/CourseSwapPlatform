#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Env, String};

#[test]
fn test_course_swap_flow() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    let contract_id = env.register_contract(None, CourseSwapContract);
    let client = CourseSwapContractClient::new(&env, &contract_id);

    // 1. Init
    client.init(&admin);

    let offered = CourseInfo {
        course_id: String::from_str(&env, "CS101"),
        course_name: String::from_str(&env, "Lap trinh Rust"),
        class_id: String::from_str(&env, "L01"),
        time_slot: String::from_str(&env, "Mon 8:00"),
    };

    let wanted = CourseInfo {
        course_id: String::from_str(&env, "MATH202"),
        course_name: String::from_str(&env, "Toan Cao Cap"),
        class_id: String::from_str(&env, "L05"),
        time_slot: String::from_str(&env, "Fri 13:00"),
    };

    // 2. User 1 post request
    let request_id = client.post_request(&user1, &offered, &wanted);
    assert_eq!(request_id, 1);

    // 3. User 2 checks available swaps (Anonymous)
    let available = client.get_available_swaps();
    assert_eq!(available.len(), 1);
    let anon_req = available.get(0).unwrap();
    assert_eq!(anon_req.id, 1);
    assert_eq!(anon_req.offered_course.course_id, String::from_str(&env, "CS101"));
    // Note: AnonymousSwapRequest does not contain 'owner' field

    // 4. User 2 tries to swap with WRONG course -> Should Error
    let wrong_course = CourseInfo {
        course_id: String::from_str(&env, "PHYS101"),
        course_name: String::from_str(&env, "Vat Ly"),
        class_id: String::from_str(&env, "L01"),
        time_slot: String::from_str(&env, "Tue 8:00"),
    };

    let result = client.try_execute_swap(&user2, &request_id, &wrong_course);
    assert!(result.is_err());

    // 5. User 2 swaps with CORRECT course
    client.execute_swap(&user2, &request_id, &wanted);

    // 6. Check status - listing should be gone from available
    let available_after = client.get_available_swaps();
    assert_eq!(available_after.len(), 0);
}

#[test]
fn test_admin_cancel() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let contract_id = env.register_contract(None, CourseSwapContract);
    let client = CourseSwapContractClient::new(&env, &contract_id);

    client.init(&admin);

    let course = CourseInfo {
        course_id: String::from_str(&env, "A"),
        course_name: String::from_str(&env, "A"),
        class_id: String::from_str(&env, "A"),
        time_slot: String::from_str(&env, "A"),
    };

    let req_id = client.post_request(&user1, &course, &course);
    assert_eq!(client.get_available_swaps().len(), 1);

    // Admin cancels
    client.admin_cancel(&admin, &req_id);
    assert_eq!(client.get_available_swaps().len(), 0);
}
