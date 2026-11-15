Write-Host "===========================================" -ForegroundColor Cyan
Write-Host " Building OwnLauncher (Frontend + Rust)" -ForegroundColor Cyan
Write-Host "===========================================" -ForegroundColor Cyan

Write-Host "`n[1/2] Building React Frontend with Vite..." -ForegroundColor Yellow
Set-Location frontend
npm run build
if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] Frontend build failed!" -ForegroundColor Red
    Set-Location ..
    exit 1
}
Set-Location ..

Write-Host "`n[2/2] Compiling standalone Rust executable..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] Cargo build failed!" -ForegroundColor Red
    exit 1
}

Write-Host "`n===========================================" -ForegroundColor Green
Write-Host " SUCCESS!" -ForegroundColor Green
Write-Host " Binary: target\release\ownlauncher.exe" -ForegroundColor Green
Write-Host "===========================================" -ForegroundColor Green
