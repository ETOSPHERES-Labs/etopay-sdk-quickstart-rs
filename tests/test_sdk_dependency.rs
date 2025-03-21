use etopay_sdk::core::Sdk;

#[test]
fn test_sdk_dependency_works() {
    println!("Get SDK build Info:");
    let build_info = Sdk::get_build_info();
    println!("{build_info}");
}
