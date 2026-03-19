@echo off
echo Uninstalling i Programming Language...
echo.

REM Get the directory where this script is located
set INSTALL_DIR=%~dp0
set INSTALL_DIR=%INSTALL_DIR:~0,-1%

REM Check if running as administrator
net session >nul 2>&1
if %errorLevel% == 0 (
    echo Running with administrator privileges...
) else (
    echo WARNING: Not running as administrator.
    echo Some features may not be removed correctly.
    echo Please run this script as administrator for full removal.
    echo.
    pause
)

REM Remove file association
echo Removing file association for .i files...
assoc .i= >nul 2>&1
if %errorLevel% == 0 (
    echo ✓ Removed .i file association
) else (
    echo ✗ Failed to remove .i file association
)

REM Remove file type definition
ftype iLangFile= >nul 2>&1
if %errorLevel% == 0 (
    echo ✓ Removed iLangFile type definition
) else (
    echo ✗ Failed to remove iLangFile type definition
)

REM Remove registry entries
echo Removing registry entries...
reg delete "HKCR\iLangFile" /f >nul 2>&1
reg delete "HKCR\Directory\Background\shell\New i File" /f >nul 2>&1

REM Remove from PATH (this is more complex, so we'll just show instructions)
echo.
echo Note: To completely remove i from your PATH:
echo 1. Open System Properties ^> Advanced ^> Environment Variables
echo 2. Find 'Path' in System variables
echo 3. Remove the entry: %INSTALL_DIR%
echo.

echo Uninstallation complete!
echo.
pause
