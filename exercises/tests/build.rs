//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
//! 

fn main() {
    // 设置 TEST_FOO 环境变量为当前时间的秒数，用于 tests7
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); 
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 启用 "pass" feature，用于 tests8
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
