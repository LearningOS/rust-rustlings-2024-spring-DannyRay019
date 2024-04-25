//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, set up an environment variable called `TEST_FOO`.
    // This variable should contain a timestamp value.
    // Here, we set it to the current timestamp.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let your_command = format!(
        "rustc-env=TEST_FOO={}",
        timestamp
    );
    println!("cargo::{}", your_command);

    // In tests8, enable the "pass" feature.
    let your_command="rustc-cfg=feature=\"pass\"";
    println!("cargo::{}",your_command);
}


