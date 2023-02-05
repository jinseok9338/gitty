#[macro_export]
macro_rules! run_cmd {
    ($cmd:expr) => {{
        use indicatif::ProgressStyle;
        use std::process::Command;
        use std::sync::mpsc;
        use std::{thread, time};

        let pb = ProgressBar::new(100);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.red} Running command...")
                .expect("Unable to set template")
                .progress_chars("#>-"),
        );

        pb.set_position(0);
        let (tx, rx): (
            mpsc::Sender<std::process::Output>,
            mpsc::Receiver<std::process::Output>,
        ) = mpsc::channel();

        let pb_clone = pb.clone();
        thread::spawn(move || {
            let output = Command::new("sh")
                .arg("-c")
                .arg($cmd)
                .output()
                .expect("Failed to execute command");
            // move the pb a bit to show that the command is running in loop
            tx.send(output).unwrap();
            pb_clone.finish_with_message("Command finished");
        });

        loop {
            match rx.try_recv() {
                Ok(_) => {
                    break;
                }
                Err(_) => {
                    thread::sleep(time::Duration::from_millis(100));
                }
            }
        }
        Ok::<(), Error>(())
    }};
}
