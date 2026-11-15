@echo off
echo ===========================================
echo  Building OwnLauncher (Frontend + Rust)
echo ===========================================

echo.
echo [1/2] Building React Frontend with Vite...
cd frontend
call npm run build
if %errorlevel% neq 0 (
    echo [ERROR] Frontend build failed!
    cd ..
    pause
    exit /b %errorlevel%
)
cd ..

echo.
echo [2/2] Compiling standalone Rust executable...
cargo build --release
if %errorlevel% neq 0 (
    echo [ERROR] Cargo release build failed!
    pause
    exit /b %errorlevel%
)

echo.
echo ===========================================
echo  SUCCESS! 
echo  Output: target\release\ownlauncher.exe
echo ===========================================
pause
