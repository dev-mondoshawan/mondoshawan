# Start Grafana and Prometheus for mondoshawan Monitoring

Write-Host "🚀 Starting mondoshawan Monitoring Stack..." -ForegroundColor Cyan

# Check if Docker is running
Write-Host "`n📦 Checking Docker..." -ForegroundColor Yellow
try {
    $dockerStatus = docker ps 2>&1
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Docker is not running!" -ForegroundColor Red
        Write-Host "`nPlease start Docker Desktop and try again." -ForegroundColor Yellow
        Write-Host "You can start it manually or run:" -ForegroundColor Yellow
        Write-Host "  Start-Process 'C:\Program Files\Docker\Docker\Docker Desktop.exe'" -ForegroundColor Gray
        exit 1
    }
    Write-Host "✅ Docker is running" -ForegroundColor Green
} catch {
    Write-Host "❌ Docker is not running!" -ForegroundColor Red
    Write-Host "Please start Docker Desktop and try again." -ForegroundColor Yellow
    exit 1
}

# Change to grafana directory
$grafanaDir = Join-Path $PSScriptRoot "grafana"
if (-not (Test-Path $grafanaDir)) {
    $grafanaDir = Join-Path (Get-Location) "grafana"
}

if (-not (Test-Path $grafanaDir)) {
    Write-Host "❌ Grafana directory not found!" -ForegroundColor Red
    Write-Host "Expected: $grafanaDir" -ForegroundColor Yellow
    exit 1
}

Set-Location $grafanaDir

# Start services
Write-Host "`n🔧 Starting Prometheus and Grafana..." -ForegroundColor Yellow
docker-compose up -d

if ($LASTEXITCODE -eq 0) {
    Write-Host "`n✅ Services started successfully!" -ForegroundColor Green
    Write-Host "`n📊 Access Grafana at: http://localhost:3000" -ForegroundColor Cyan
    Write-Host "   Username: admin" -ForegroundColor Gray
    Write-Host "   Password: admin" -ForegroundColor Gray
    Write-Host "`n📈 Access Prometheus at: http://localhost:9090" -ForegroundColor Cyan
    Write-Host "`n💡 Make sure your mondoshawan node is running and exposing metrics at:" -ForegroundColor Yellow
    Write-Host "   http://localhost:8545/metrics" -ForegroundColor Gray
    Write-Host "`n📋 To view logs: docker-compose logs -f" -ForegroundColor Gray
    Write-Host "🛑 To stop: docker-compose down" -ForegroundColor Gray
} else {
    Write-Host "`n❌ Failed to start services!" -ForegroundColor Red
    Write-Host "Check the error messages above." -ForegroundColor Yellow
    exit 1
}
