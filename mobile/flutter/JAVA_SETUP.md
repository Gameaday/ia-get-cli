# Java Setup for Flutter Android Development

## CI/CD (GitHub Actions)
Java is automatically configured via `setup-java@v4` action:
- Distribution: Zulu OpenJDK
- Version: 17 (LTS)
- JAVA_HOME is set automatically

## Local Development

### Option 1: Use Android Studio's Bundled JDK (Recommended)
Android Studio includes a compatible JDK. No additional setup needed.

### Option 2: Manual JDK Installation

#### Windows
1. Install JDK 17 or 21 from [Adoptium](https://adoptium.net/) or [Azul Zulu](https://www.azul.com/downloads/)
2. Set JAVA_HOME environment variable:
   ```powershell
   $env:JAVA_HOME = "C:\Program Files\Java\jdk-17"
   # Or add permanently via System Properties
   ```

#### macOS
```bash
# Install via Homebrew
brew install openjdk@17

# Set JAVA_HOME in ~/.zshrc or ~/.bash_profile
export JAVA_HOME=$(/usr/libexec/java_home -v 17)
```

#### Linux (Ubuntu/Debian)
```bash
# Install
sudo apt install openjdk-17-jdk

# Set JAVA_HOME in ~/.bashrc or ~/.profile
export JAVA_HOME=/usr/lib/jvm/java-17-openjdk-amd64
```

### Verify Java Setup
```bash
# Check Java version
java -version

# Check JAVA_HOME
echo $JAVA_HOME  # Linux/macOS
echo %JAVA_HOME%  # Windows CMD
$env:JAVA_HOME   # Windows PowerShell

# Flutter should detect Java
flutter doctor -v
```

## Gradle Configuration

The `android/gradle.properties` file should **NOT** hardcode `org.gradle.java.home`.
Let Gradle use JAVA_HOME from the environment instead.

**❌ Don't do this:**
```properties
org.gradle.java.home=C:\\Program Files\\Java\\jdk-21
```

**✅ Do this:**
```properties
# No org.gradle.java.home line - use JAVA_HOME from environment
```

This ensures builds work on:
- ✅ Local development (Windows/macOS/Linux)
- ✅ CI/CD (GitHub Actions, GitLab CI, etc.)
- ✅ Different machines with different JDK paths

## Troubleshooting

### "Java home supplied is invalid"
- Remove `org.gradle.java.home` from `android/gradle.properties`
- Ensure JAVA_HOME is set in your environment
- Restart your IDE/terminal after setting JAVA_HOME

### "Unsupported Java version"
- Flutter requires Java 11, 17, or 21
- Use `java -version` to check your version
- Install a compatible version if needed

### Android Studio doesn't detect JDK
- Go to File → Project Structure → SDK Location
- Set JDK location to your installed JDK
- Android Studio can also download a JDK for you
