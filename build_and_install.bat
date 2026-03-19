@echo off
echo Building and Installing i Programming Language...
echo.

REM Step 1: Build the release version
echo Building release version...
cargo build --release
if %errorLevel% neq 0 (
    echo ✗ Build failed!
    pause
    exit /b 1
)
echo ✓ Build successful!

REM Step 2: Run the installer
echo.
echo Running installer...
call install.bat

echo.
echo All done! Your i programming language is ready to use.
pause
