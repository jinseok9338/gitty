#[macro_export]
macro_rules! run_cmd {
    ($cmd:expr) => {{
        use std::process::Command;
        let output = Command::new("sh")
            .arg("-c")
            .arg($cmd)
            .output();
          

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            let status = output.status;
            if status.success() {
                Ok(stdout.to_string())
            } else {
                Err(stderr.to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }}
    };
}