# Flutter Android Build Environment Setup
# This script configures the Java environment for Flutter Android builds
# Usage: . .\scripts\setup-flutter-env.ps1

Write-Host "Setting up Flutter Android build environment..."

# Set Java 21 for Android builds (Java 25 is unsupported)
$env:JAVA_HOME = "C:\Program Files\Java\jdk-21"
$env:PATH = "C:\Program Files\Java\jdk-21\bin;$env:PATH"

Write-Host "✓ JAVA_HOME set to: $env:JAVA_HOME"

# Verify Java version
$javaVersion = & java -version 2>&1 | Select-Object -First 1
Write-Host "✓ Java version: $javaVersion"

# Stop any running Gradle daemons to pick up new Java version
Write-Host "Stopping Gradle daemons..."
Set-Location "mobile\flutter\android"
& .\gradlew --stop 2>&1 | Out-Null
Set-Location "..\..\.."
Write-Host "✓ Gradle daemons stopped"

Write-Host ""
Write-Host "Environment ready for Flutter Android builds!"
Write-Host "You can now run Flutter build commands."
