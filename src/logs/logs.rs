// make struct for logs and impliment the logs

pub struct Logs {
    id : Option<i32>,
    message : Option<String>,
    created_at : Option<String>,
    username : Option<String>,
    email : Option<String>,
    branch : Option<String>,
 
}

impl Logs {
    pub fn new() -> Self {
        Self {
            id : None,
            message : None,
            created_at : None,
            username : None,
            email : None,
            branch : None,
        }
    }

    pub fn write_logs(&self) {
        //write logs as Json to the file

        
    }
}
