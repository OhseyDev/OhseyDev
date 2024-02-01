use std::{
    str::from_utf8,
    env,
    process::Command,
    fs::File,
    io::Write,
    io::Result as IoResult
};

use uuid::Uuid;

fn main() -> IoResult<()> {
    let mut path = env::current_dir()?;
    path.push("android");
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg(path.as_os_str())
            .arg(".\\gradlew.bat build")
            .output()?//.expect("Failed to build Android app")
    } else {
        path.push("gradlew");
        Command::new("sh")
            .arg("-c")
            .arg(format!("{} build", path.display()))
            .output()?//.expect("Failed to build Android app")
    };
    if output.status.success() {
        let s = match from_utf8(&output.stderr) {
            Ok(v) => v,
            Err(e) => panic!("Unable to print stderr output of command: {}", e)
        };
        println!("{}", s);
        println!("Successfully built Android app");
    } else {
        let s = match from_utf8(&output.stderr) {
            Ok(v) => v,
            Err(e) => panic!("Unable to print stderr output of command: {}", e)
        };
        println!("{}", s);
    }
    Ok(())
}

