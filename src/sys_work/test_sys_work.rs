#[cfg(test)]
mod sys_work_tests {
    use super::super::sys_work::*;
    use std::{env, path::PathBuf};

    #[test]
    fn check_dir() {
        let inputs = vec![
            (PathBuf::from("test"), false),
            (PathBuf::from("./"), true),
            (PathBuf::from("."), true),
            (PathBuf::from("../"), true),
        ];

        let sys_work = SysWork {};
        for (path, result) in inputs {
            assert_eq!(sys_work.check_dir(&path).unwrap(), result);
        }
    }

    #[test]
    fn current_dir() {
        let inputs = vec![
            ("test", env::current_dir().unwrap()),
            ("./", PathBuf::from("./")),
            (".", PathBuf::from(".")),
            ("../", PathBuf::from("../")),
        ];

        let sys_work = SysWork {};
        for (path, result) in inputs {
            assert_eq!(sys_work.currnet_dir(&path).unwrap(), result);
        }
    }
}
