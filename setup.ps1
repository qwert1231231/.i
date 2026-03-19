# PowerShell Installer Script for i Programming Language
# This script will automatically run after git clone

Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing

# Create the main form
$form = New-Object System.Windows.Forms.Form
$form.Text = "i Programming Language Setup"
$form.Size = New-Object System.Drawing.Size(450, 200)
$form.StartPosition = "CenterScreen"
$form.FormBorderStyle = "FixedDialog"
$form.MaximizeBox = $false
$form.MinimizeBox = $false

# Add icon if available (optional)
try {
    $iconPath = Join-Path $PSScriptRoot "logo.ico"
    if (Test-Path $iconPath) {
        $form.Icon = [System.Drawing.Icon]::ExtractAssociatedIcon($iconPath)
    }
} catch {
    # Continue without icon if not found
}

# Title label
$titleLabel = New-Object System.Windows.Forms.Label
$titleLabel.Text = "i Programming Language"
$titleLabel.Font = New-Object System.Drawing.Font("Arial", 14, [System.Drawing.FontStyle]::Bold)
$titleLabel.ForeColor = [System.Drawing.Color]::FromArgb(255, 0, 0)  # Red color
$titleLabel.Location = New-Object System.Drawing.Point(20, 20)
$titleLabel.Size = New-Object System.Drawing.Size(400, 30)
$form.Controls.Add($titleLabel)

# Description label
$descLabel = New-Object System.Windows.Forms.Label
$descLabel.Text = "Would you like to install i language and add .i file support to your system?`nThis will enable double-click execution and right-click context menus."
$descLabel.Location = New-Object System.Drawing.Point(20, 60)
$descLabel.Size = New-Object System.Drawing.Size(400, 60)
$descLabel.ForeColor = [System.Drawing.Color]::Black
$form.Controls.Add($descLabel)

# Yes button
$yesButton = New-Object System.Windows.Forms.Button
$yesButton.Text = "Yes, Install i Language"
$yesButton.Location = New-Object System.Drawing.Point(50, 120)
$yesButton.Size = New-Object System.Drawing.Size(150, 35)
$yesButton.BackColor = [System.Drawing.Color]::FromArgb(76, 175, 80)  # Green
$yesButton.ForeColor = [System.Drawing.Color]::White
$yesButton.Font = New-Object System.Drawing.Font("Arial", 10, [System.Drawing.FontStyle]::Bold)
$form.Controls.Add($yesButton)

# No button
$noButton = New-Object System.Windows.Forms.Button
$noButton.Text = "No, Thanks"
$noButton.Location = New-Object System.Drawing.Point(250, 120)
$noButton.Size = New-Object System.Drawing.Size(120, 35)
$noButton.BackColor = [System.Drawing.Color]::FromArgb(244, 67, 54)  # Red
$noButton.ForeColor = [System.Drawing.Color]::White
$noButton.Font = New-Object System.Drawing.Font("Arial", 10, [System.Drawing.FontStyle]::Bold)
$form.Controls.Add($noButton)

# Button click handlers
$yesButton.Add_Click({
    $form.Close()
    
    # Launch UAC installer instead of direct installation
    $scriptDir = $PSScriptRoot
    $uacScript = Join-Path $scriptDir "install_with_uac.bat"
    
    if (Test-Path $uacScript) {
        Write-Host "Launching UAC installer..."
        Start-Process cmd.exe "/c `"$uacScript`"" -Wait
    } else {
        # Fallback to UAC PowerShell launcher
        $uacPsScript = Join-Path $scriptDir "uac_launcher.ps1"
        if (Test-Path $uacPsScript) {
            Write-Host "Launching UAC PowerShell installer..."
            Start-Process powershell.exe "-ExecutionPolicy Bypass -File `"$uacPsScript`"" -Wait
        } else {
            [System.Windows.Forms.MessageBox]::Show(
                "UAC installer not found. Please run 'install_with_uac.bat' manually as Administrator.", 
                "Installation Error", 
                [System.Windows.Forms.MessageBoxButtons]::OK, 
                [System.Windows.Forms.MessageBoxIcon]::Error
            )
        }
    }
})

$noButton.Add_Click({
    $form.Close()
    
    [System.Windows.Forms.MessageBox]::Show(
        "i Programming Language was not installed.`n`nTo install later, run 'build_and_install.bat' as Administrator.", 
        "Installation Cancelled", 
        [System.Windows.Forms.MessageBoxButtons]::OK, 
        [System.Windows.Forms.MessageBoxIcon]::Information
    )
})

# Check if script was called with -Install parameter (for admin restart)
if ($args -contains "-Install") {
    $scriptDir = $PSScriptRoot
    $installScript = Join-Path $scriptDir "install.bat"
    
    if (Test-Path $installScript) {
        Write-Host "Installing i programming language..."
        & cmd.exe /c "`"$installScript`""
        
        [System.Windows.Forms.MessageBox]::Show(
            "i Programming Language has been successfully installed!`n`nYou can now:`n• Double-click .i files to run them`n• Right-click to create new .i files`n• Use 'i filename.i' from command line", 
            "Installation Complete", 
            [System.Windows.Forms.MessageBoxButtons]::OK, 
            [System.Windows.Forms.MessageBoxIcon]::Information
        )
    }
    exit
}

# Show the form
[void]$form.ShowDialog()
