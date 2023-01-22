use std::{env, path::PathBuf, error::Error};

pub struct SysWork {

}
 
impl SysWork{
    pub fn check_dir(&self,directory:&PathBuf) -> Result<bool,Box<dyn Error>> {
        // check if the current directory is provided and check if it's the right one
        if  directory.exists() && directory.is_dir()   {
            return Ok(true);
        } else {
            return Ok(false);
        }   
        }
    
    pub fn currnet_dir(&self,directory:&str) -> Result<PathBuf,Box<dyn Error>> {
     
        let directory = PathBuf::from(directory);
        if self.check_dir(&directory).unwrap() {
            return Ok(directory.into());
        }
        return Ok(env::current_dir()?);
    }


}
