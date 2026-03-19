@echo off
echo Installing i Programming Language...
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
    echo Some features may not work correctly.
    echo Please run this script as administrator for full functionality.
    echo.
    pause
)

REM Add to PATH
echo Adding i to system PATH...
setx PATH "%PATH%;%INSTALL_DIR%" /M >nul 2>&1
if %errorLevel% == 0 (
    echo ✓ Added to system PATH
) else (
    echo ✗ Failed to add to PATH (may require admin privileges)
)

REM Create file association for .i files
echo Creating file association for .i files...
assoc .i=iLangFile >nul 2>&1
if %errorLevel% == 0 (
    echo ✓ Associated .i extension
) else (
    echo ✗ Failed to associate .i extension (may require admin privileges)
)

REM Set the file type description
ftype iLangFile="%INSTALL_DIR%\target\release\i.exe" "%%1" %%* >nul 2>&1
if %errorLevel% == 0 (
    echo ✓ Set default program for .i files
) else (
    echo ✗ Failed to set default program (may require admin privileges)
)

REM Create registry entries for better integration
echo Creating registry entries...
reg add "HKCR\iLangFile" /v "FriendlyTypeName" /t REG_SZ /d "i Language Source File" /f >nul 2>&1
reg add "HKCR\iLangFile" /v "PerceivedType" /t REG_SZ /d "text" /f >nul 2>&1
reg add "HKCR\iLangFile\DefaultIcon" /v "" /t REG_SZ /d "%INSTALL_DIR%\logo.ico,0" /f >nul 2>&1

REM Add context menu option to run .i files
reg add "HKCR\iLangFile\shell\Run" /v "" /t REG_SZ /d "Run with i" /f >nul 2>&1
reg add "HKCR\iLangFile\shell\Run\command" /v "" /t REG_SZ /d "\"%INSTALL_DIR%\target\release\i.exe\" \"%%1\"" /f >nul 2>&1

REM Add context menu option to create new .i file
reg add "HKCR\Directory\Background\shell\New i File" /v "" /t REG_SZ /d "New i Language File" /f >nul 2>&1
reg add "HKCR\Directory\Background\shell\New i File\command" /v "" /t REG_SZ /d "notepad.exe \"%%V\%%w.i\"" /f >nul 2>&1

echo.
echo Installation complete!
echo.
echo You can now:
echo - Run .i files by double-clicking them
echo - Right-click in any folder to create a new .i file
echo - Use 'i filename.i' from command line anywhere
echo.
echo Note: Make sure to build the release version first:
echo   cargo build --release
echo.
pause
