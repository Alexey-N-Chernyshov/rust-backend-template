fn get_git_pretty_version() -> Option<String> {
    use std::process::Command;

    let git_command = &Command::new("git")
        .arg("describe")
        .arg("--tags")
        .arg("--always")
        .output();

    if let Ok(git_output) = git_command {
        let git_string = String::from_utf8_lossy(&git_output.stdout);
        return Some(
            git_string
                .lines()
                .next()
                .unwrap_or("git commit unknown")
                .to_string(),
        );
    }

    None
}

fn main() {
    if let Some(git_pretty) = get_git_pretty_version() {
        println!("cargo:rustc-env=GIT_PRETTY_VERSION={git_pretty}");
        println!(
            "cargo:rustc-env=BUILD_TIMESTAMP={:?}",
            chrono::offset::Utc::now()
        );
    } else {
        println!("cargo:rustc-env=GIT_PRETTY_VERSION=unknown");
        println!("cargo:rustc-env=BUILD_TIMESTAMP=unknown");
    }
}
