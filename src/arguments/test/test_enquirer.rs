#[cfg(test)]
mod sys_work_tests {

    use super::super::super::enquirer::*;

    #[test]
    fn validate_dir() {
        let inputs = vec![
            (
                Enquirer {
                 
                    directory: Option::from("test".to_owned()),
                    url: Option::from("test".to_owned()),
                },
                false,
            ),
            (
                Enquirer {
              
                    directory: Option::from("./".to_owned()),
                    url: Option::from("test".to_owned()),
                },
                true,
            ),
            (
                Enquirer {
            
                    directory: Option::from(".".to_owned()),
                    url: Option::from("test".to_owned()),
                },
                true,
            ),
            (
                Enquirer {
                 
                    directory: Option::from("../".to_owned()),
                    url: Option::from("test".to_owned()),
                },
                true,
            ),
        ];

        for (enquirer, result) in inputs {
            println!("{:?},- {}", enquirer, result);
            assert_eq!(enquirer.validate_directory(), result);
        }
    }

    #[test]
    fn validate_url() {
        let inputs = vec![
            (
                Enquirer {
                  
                    directory: Option::from("test".to_owned()),
                    url: Option::from("test".to_owned()),
                },
                false,
            ),
            (
                Enquirer {
       
                    directory: Option::from("./".to_owned()),
                    url: Option::from("https://www.github.com".to_owned()),
                },
                true,
            ),
            (
                Enquirer {
               
                    directory: Option::from(".".to_owned()),
                    url: Option::from("http://www.github.com".to_owned()),
                },
                true,
            ),
            (
                Enquirer {
                  
                    directory: Option::from("../".to_owned()),
                    url: Option::from("www.google.com".to_owned()),
                },
                false,
            ),
        ];

        for (enquirer, result) in inputs {
            println!("{:?},- {}", enquirer, result);
            assert_eq!(enquirer.validate_url(), result);
        }
    }
}
