extern crate winapi;

use std::io::Error;
use std::ptr;

use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::winnt::{TokenElevation, HANDLE, TOKEN_ELEVATION, TOKEN_QUERY};

use std::io;
use std::process::Command;

struct QueryAccessToken(HANDLE);

impl QueryAccessToken {
    pub fn from_current_process() -> Result<Self, Error> {
        unsafe {
            let mut handle: HANDLE = ptr::null_mut();
            if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut handle) != 0 {
                Ok(Self(handle))
            } else {
                Err(Error::last_os_error())
            }
        }
    }

    /// On success returns a bool indicating if the access token has elevated privilidges.
    /// Otherwise returns an OS error.
    pub fn is_elevated(&self) -> Result<bool, Error> {
        unsafe {
            let mut elevation = TOKEN_ELEVATION::default();
            let size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
            let mut ret_size = size;
            // The weird looking repetition of `as *mut _` is casting the reference to a c_void pointer.
            if GetTokenInformation(
                self.0,
                TokenElevation,
                &mut elevation as *mut _ as *mut _,
                size,
                &mut ret_size,
            ) != 0
            {
                Ok(elevation.TokenIsElevated != 0)
            } else {
                Err(Error::last_os_error())
            }
        }
    }
}

impl Drop for QueryAccessToken {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { CloseHandle(self.0) };
        }
    }
}

pub struct Auxiliary {}

impl Auxiliary {
    pub fn new() -> Auxiliary {
        Auxiliary {}
    }

    pub fn is_elevated(&self) -> bool {
        let query_access_token = QueryAccessToken::from_current_process().unwrap();
        query_access_token.is_elevated().unwrap()
    }

    pub fn get_current_user_name(&self) -> String {
        //Code to get the current user name

        whoami::username()
    }

    pub fn run_powershell_script(&self, script_path: &str, file_name: &str) -> io::Result<()> {
        let output = Command::new("powershell")
            .arg("-ExecutionPolicy")
            .arg("Unrestricted") // You may need to adjust this depending on your script's requirements
            .arg("-File")
            .arg("autorun.ps1")
            .arg(script_path)
            .arg(file_name)
            .output()?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to execute PowerShell script",
            ));
        }

        Ok(())
    }
}
