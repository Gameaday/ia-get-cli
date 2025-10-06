# Flutter Android Build Setup Script
# Sets the correct Java environment for building Flutter Android apps

Write-Host "🔧 Setting up Flutter Android build environment..." -ForegroundColor Cyan

# Set JAVA_HOME to Android Studio's bundled JDK (Java 17)
$androidStudioJbr = "C:\Program Files\Android\Android Studio\jbr"

if (Test-Path $androidStudioJbr) {
    $env:JAVA_HOME = $androidStudioJbr
    Write-Host "✅ JAVA_HOME set to: $androidStudioJbr" -ForegroundColor Green
    
    # Verify Java version
    $javaVersion = & "$androidStudioJbr\bin\java" -version 2>&1 | Select-String -Pattern "version"
    Write-Host "📦 Java version: $javaVersion" -ForegroundColor Yellow
} else {
    Write-Host "❌ Android Studio JBR not found at: $androidStudioJbr" -ForegroundColor Red
    Write-Host "Please install Android Studio or update the path in this script." -ForegroundColor Red
    exit 1
}

# Stop any running Gradle daemons
Write-Host "`n🛑 Stopping Gradle daemons..." -ForegroundColor Cyan
Push-Location android
& .\gradlew --stop 2>&1 | Out-Null
Pop-Location
Write-Host "✅ Gradle daemons stopped" -ForegroundColor Green

Write-Host "`n✨ Environment ready! You can now run:" -ForegroundColor Green
Write-Host "   flutter build apk" -ForegroundColor White
Write-Host "   flutter build appbundle" -ForegroundColor White
Write-Host "   flutter build apk --release" -ForegroundColor White
