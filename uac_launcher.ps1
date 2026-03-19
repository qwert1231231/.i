# UAC Launcher for i Language Installation
# This script triggers the Windows UAC prompt

param(
    [string]$InstallScript = "install.bat",
    [string]$WorkingDir = ""
)

# Get current directory if not specified
if ([string]::IsNullOrEmpty($WorkingDir)) {
    $WorkingDir = Split-Path -Parent $MyInvocation.MyCommand.Path
}

# Check if already running as administrator
$isAdmin = ([System.Security.Principal.WindowsPrincipal] [System.Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([System.Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    # Not running as admin - relaunch with UAC prompt
    try {
        # Get the path to current script
        $scriptPath = $MyInvocation.MyCommand.Path
        
        # Build arguments for relaunch
        $arguments = "-File `"$scriptPath`" -InstallScript `"$InstallScript`" -WorkingDir `"$WorkingDir`""
        
        # Start new PowerShell process with admin rights (triggers UAC)
        Start-Process powershell.exe -ArgumentList $arguments -Verb RunAs -Wait
        
        # Exit after admin process completes
        exit
    }
    catch {
        # User clicked "No" on UAC prompt or cancelled
        [System.Windows.Forms.MessageBox]::Show(
            "Installation requires administrator privileges to modify system settings.`n`nPlease run the installer as Administrator to continue.",
            "Installation Cancelled",
            [System.Windows.Forms.MessageBoxButtons]::OK,
            [System.Windows.Forms.MessageBoxIcon]::Warning
        )
        exit 1
    }
} else {
    # Running as admin - proceed with installation
    Write-Host "Running with administrator privileges..."
    Write-Host "Working directory: $WorkingDir"
    
    # Change to working directory
    Set-Location $WorkingDir
    
    # Check if install script exists
    $installPath = Join-Path $WorkingDir $InstallScript
    if (Test-Path $installPath) {
        Write-Host "Running installation script: $InstallScript"
        
        # Run the batch file
        $process = Start-Process cmd.exe -ArgumentList "/c", "`"$installPath`"" -Wait -PassThru
        
        if ($process.ExitCode -eq 0) {
            # Show success message
            Add-Type -AssemblyName System.Windows.Forms
            [System.Windows.Forms.MessageBox]::Show(
                "i Programming Language has been successfully installed!`n`nYou can now:`n• Double-click .i files to run them`n• Right-click to create new .i files`n• Use 'i filename.i' from command line",
                "Installation Complete",
                [System.Windows.Forms.MessageBoxButtons]::OK,
                [System.Windows.Forms.MessageBoxIcon]::Information
            )
        } else {
            # Show error message
            Add-Type -AssemblyName System.Windows.Forms
            [System.Windows.Forms.MessageBox]::Show(
                "Installation failed with exit code $($process.ExitCode).`n`nPlease check the installation log and try again.",
                "Installation Error",
                [System.Windows.Forms.MessageBoxButtons]::OK,
                [System.Windows.Forms.MessageBoxIcon]::Error
            )
        }
    } else {
        Add-Type -AssemblyName System.Windows.Forms
        [System.Windows.Forms.MessageBox]::Show(
            "Installation script not found: $InstallScript`n`nPlease ensure the installer files are present.",
            "Installation Error",
            [System.Windows.Forms.MessageBoxButtons]::OK,
            [System.Windows.Forms.MessageBoxIcon]::Error
        )
    }
}
