extern crate android_cli as android;

use std::env;

fn main() {
    let mut path = env::current_dir().expect("Failed to acquire working directory");
    path.push("android");
    android::create_dot_android(path.as_path(), String::from("Android app"), String::from("me.ohsey.android"), None).expect("Unable to prepare for building of Android app");
    let release = {
        let prof = env::var("PROFILE").unwrap_or("debug".to_string()).to_lowercase();
        prof == "release"
    };
    let status = android::trigger_build(release).expect("Failed to build Android app");
    if status.success() {
        println!("Successfully built Android app");
    } else {
        panic!("Failed to build Android app");
    }
}

