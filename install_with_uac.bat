@echo off
echo i Programming Language Installer with UAC
echo ========================================
echo.

REM Get the directory where this script is located
set INSTALL_DIR=%~dp0
set INSTALL_DIR=%INSTALL_DIR:~0,-1%

REM Check if PowerShell is available
where powershell >nul 2>&1
if %errorLevel% neq 0 (
    echo ERROR: PowerShell not found. Cannot proceed with installation.
    echo Please install PowerShell or run install.bat manually as Administrator.
    pause
    exit /b 1
)

REM Check if UAC launcher script exists
set UAC_SCRIPT=%INSTALL_DIR%\uac_launcher.ps1
if not exist "%UAC_SCRIPT%" (
    echo ERROR: UAC launcher not found.
    echo Please ensure all installer files are present.
    pause
    exit /b 1
)

echo Starting installation with administrator privileges...
echo This will trigger the Windows UAC prompt.
echo.

REM Run the UAC launcher (will trigger UAC if not admin)
powershell.exe -ExecutionPolicy Bypass -File "%UAC_SCRIPT%" -InstallScript "install.bat" -WorkingDir "%INSTALL_DIR%"

echo.
echo Installation process completed.
pause
