@echo off
echo Setting up i Programming Language...
echo.

REM Check if PowerShell is available
where powershell >nul 2>&1
if %errorLevel% neq 0 (
    echo PowerShell not found. Please run setup.ps1 manually.
    pause
    exit /b 1
)

REM Run the PowerShell setup script
echo Starting interactive setup...
powershell.exe -ExecutionPolicy Bypass -File "%~dp0setup.ps1"

echo.
echo Setup process completed.
pause
