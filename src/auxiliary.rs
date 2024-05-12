use std::process::Command;
use std::io;

pub struct Auxiliary
{
}

impl Auxiliary
{
    pub fn new() -> Auxiliary
    {
        Auxiliary
        {
        }
    }

    pub fn get_current_user_name(&self) -> String
    {
        //Code to get the current user name
        
        whoami::username()
    }
    
    pub fn run_powershell_script(&self, script_path: &str, file_name: &str) -> io::Result<()> 
    {
        let output = Command::new("powershell")
            .arg("-ExecutionPolicy")
            .arg("Unrestricted") // You may need to adjust this depending on your script's requirements
            .arg("-File")
            .arg("autorun.ps1")
            .arg(script_path)
            .arg(file_name)
            .output()?;

        if !output.status.success() 
        {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to execute PowerShell script"));
        }

        Ok(())
    }
}