# Testing Guide

Comprehensive guide for testing the ia-get project (Rust CLI + Flutter mobile app).

---

## 🧪 Flutter Mobile App Testing

### Test Structure

```
test/
├── models/                    # Model tests
│   ├── download_progress_test.dart
│   └── archive_metadata_test.dart
├── services/                  # Service tests
│   ├── download_service_test.dart
│   ├── advanced_search_service_test.dart
│   ├── search_history_service_test.dart.skip  # Database (device only)
│   └── saved_search_service_test.dart.skip    # Database (device only)
├── utils/                     # Utility tests
│   └── formatters_test.dart
└── widgets/                   # Widget tests
    └── download_card_test.dart
```

### Running Tests

```bash
cd mobile/flutter

# Run all VM tests (144 tests, ~10 seconds)
flutter test

# Run specific test file
flutter test test/services/download_service_test.dart

# Run with coverage
flutter test --coverage

# View coverage report
genhtml coverage/lcov.info -o coverage/html
open coverage/html/index.html

# Run tests matching pattern
flutter test --name "should download file"

# Verbose output
flutter test --verbose
```

### Test Categories

#### VM Tests (144 tests) ✅
Run in Flutter VM, no device required.

**Services (70+ tests):**
- `download_service_test.dart` - Download logic
- `advanced_search_service_test.dart` - Search filters (26 tests)
- `settings_service_test.dart` - Settings persistence

**Models (40+ tests):**
- `download_progress_test.dart` - Progress tracking
- `archive_metadata_test.dart` - Metadata parsing
- `download_task_test.dart` - Resume state

**Utils (20+ tests):**
- `formatters_test.dart` - Date, size, duration formatting
- `validators_test.dart` - Input validation

**Widgets (10+ tests):**
- `download_card_test.dart` - Download UI component
- `search_filters_test.dart` - Filter UI

#### Database Tests (39 tests) 📱
Require device/emulator, use SQLite.

**Files (skipped in CI):**
- `search_history_service_test.dart.skip` (19 tests)
- `saved_search_service_test.dart.skip` (20 tests)

**Running on Device:**
```bash
# Rename to enable
mv test/services/search_history_service_test.dart.skip \
   test/services/search_history_service_test.dart

# Run on connected device
flutter test --device-id <device-id>

# Rename back to skip in CI
mv test/services/search_history_service_test.dart \
   test/services/search_history_service_test.dart.skip
```

**Why Skip in CI?**
- `sqflite` package requires platform-specific native code
- CI VM doesn't have Android/iOS runtime
- Tests pass on physical devices/emulators

### Test Framework

**Packages:**
- `flutter_test` - Core testing framework
- `mockito ^5.4.4` - Mocking HTTP, services
- `build_runner` - Code generation for mocks

**Generating Mocks:**
```bash
flutter pub run build_runner build
```

### Writing Tests

**Example Service Test:**
```dart
import 'package:flutter_test/flutter_test.dart';
import 'package:mockito/mockito.dart';
import 'package:mockito/annotations.dart';

@GenerateMocks([HttpClient])
void main() {
  group('DownloadService', () {
    late DownloadService service;
    late MockHttpClient mockClient;

    setUp(() {
      mockClient = MockHttpClient();
      service = DownloadService(client: mockClient);
    });

    tearDown(() {
      service.dispose();
    });

    test('should download file successfully', () async {
      // Arrange
      when(mockClient.get(any))
        .thenAnswer((_) async => Response(data: 'test', statusCode: 200));

      // Act
      final result = await service.downloadFile('test.txt');

      // Assert
      expect(result.isSuccess, true);
      verify(mockClient.get(any)).called(1);
    });
  });
}
```

**Example Widget Test:**
```dart
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';

void main() {
  testWidgets('DownloadCard shows progress', (tester) async {
    // Arrange
    await tester.pumpWidget(
      MaterialApp(
        home: DownloadCard(
          progress: DownloadProgress(percentage: 0.5),
        ),
      ),
    );

    // Act
    await tester.pump();

    // Assert
    expect(find.text('50%'), findsOneWidget);
    expect(find.byType(LinearProgressIndicator), findsOneWidget);
  });
}
```

### Common Test Patterns

**1. Testing Async Operations:**
```dart
test('should handle async operation', () async {
  final result = await service.fetchData();
  expect(result, isNotNull);
});
```

**2. Testing Streams:**
```dart
test('should emit progress updates', () {
  expectLater(
    service.progressStream,
    emitsInOrder([0.0, 0.5, 1.0]),
  );
  service.startDownload();
});
```

**3. Testing Errors:**
```dart
test('should throw error on invalid input', () {
  expect(
    () => service.process(''),
    throwsA(isA<ValidationError>()),
  );
});
```

**4. Mocking HTTP Responses:**
```dart
when(mockClient.get(any))
  .thenAnswer((_) async => Response(
    data: jsonEncode({'key': 'value'}),
    statusCode: 200,
  ));
```

### Test Coverage

**Current Coverage:** ~70%

**High Coverage Areas:**
- Services: 85%
- Models: 90%
- Utils: 95%

**Lower Coverage Areas:**
- Widgets: 60%
- Screens: 50%

**Coverage Goals:**
- Overall: >70%
- Critical services: >80%
- Utils/models: >90%

---

## 🦀 Rust CLI Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_download

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode (faster)
cargo test --release

# Run integration tests only
cargo test --test integration_tests
```

### Test Structure

```
tests/
├── integration_tests.rs       # End-to-end tests
├── metadata_tests_simple.rs   # Metadata parsing
└── *.rs.disabled              # Disabled tests

src/
└── lib.rs                     # Unit tests inline
```

### Benchmarking

```bash
# Run benchmarks
cargo bench

# Specific benchmark
cargo bench download_performance

# Save baseline
cargo bench -- --save-baseline main
```

**Benchmark Suite:**
- `download_performance.rs` - Download speed tests
- `performance_benchmarks.rs` - General performance

---

## 🔄 CI/CD Testing

### GitHub Actions Workflows

**Rust CI** (`.github/workflows/rust-ci.yml`):
```yaml
- name: Run tests
  run: cargo test --verbose

- name: Run lints
  run: |
    cargo clippy -- -D warnings
    cargo fmt --check
```

**Flutter CI** (`.github/workflows/flutter-ci.yml`):
```yaml
- name: Analyze code
  run: flutter analyze

- name: Run tests
  run: flutter test
```

### Test Execution in CI

**VM Tests:** All 144 tests run automatically
- Fast execution (~10 seconds)
- No device required
- Reliable on all platforms

**Database Tests:** Skipped (`.skip` extension)
- Require Android/iOS runtime
- Run manually on devices
- Not blocking for CI

### CI Test Results

**Latest Build:**
- ✅ 144/144 VM tests passing
- ✅ 0 compilation errors
- ✅ 0 analysis issues
- ⏭️ 39 database tests skipped (device testing)

---

## 🐛 Debugging Tests

### Flutter Test Debugging

**Print Debugging:**
```dart
test('should work', () {
  print('Debug: $value');  // Use print(), not debugPrint()
  expect(value, equals(42));
});

// Run with --verbose to see prints
flutter test --verbose
```

**VS Code Debugging:**
1. Set breakpoint in test
2. Right-click test file
3. Select "Debug Tests"

**Common Issues:**

**1. Async Test Timeout:**
```dart
// Increase timeout
test('long operation', () async {
  // test code
}, timeout: Timeout(Duration(seconds: 30)));
```

**2. Pump Widget Issues:**
```dart
// Wait for async operations
await tester.pumpAndSettle();

// Or pump with duration
await tester.pump(Duration(seconds: 1));
```

**3. Mock Not Being Called:**
```dart
// Verify mock was called
verify(mock.method()).called(1);

// Check what was actually called
verifyNever(mock.otherMethod());
```

### Rust Test Debugging

**Print Debugging:**
```rust
#[test]
fn test_something() {
    let value = 42;
    println!("Debug: {}", value);  // Shown with --nocapture
    assert_eq!(value, 42);
}
```

**GDB/LLDB Debugging:**
```bash
# Build with debug symbols
cargo build --tests

# Run with debugger
rust-gdb target/debug/deps/ia_get-<hash>
```

---

## 📊 Test Metrics

### Current Status (October 2025)

| Metric | Rust CLI | Flutter Mobile |
|--------|----------|----------------|
| Total Tests | ~50 | 183 (144 VM + 39 device) |
| Passing | 50/50 | 144/144 (VM) |
| Coverage | ~60% | ~70% |
| Execution Time | ~5s | ~10s (VM only) |
| CI Integration | ✅ Yes | ✅ Yes |

### Quality Gates

**Pre-Merge Requirements:**
- ✅ All VM tests pass
- ✅ No `flutter analyze` issues
- ✅ Code formatted (`dart format .`)
- ✅ No compilation errors
- ⚠️ Database tests (manual device testing)

**Pre-Release Requirements:**
- ✅ All tests pass (VM + device)
- ✅ Integration tests pass
- ✅ Manual QA on physical device
- ✅ Performance benchmarks acceptable
- ✅ Build succeeds on all platforms

---

## 🚀 Best Practices

### Test Naming
```dart
// Good
test('should download file successfully when URL is valid')
test('should throw error when file not found')

// Bad
test('test1')
test('download works')
```

### Test Organization
```dart
group('DownloadService', () {
  group('downloadFile', () {
    test('should succeed with valid URL', () {});
    test('should fail with invalid URL', () {});
    test('should retry on network error', () {});
  });
  
  group('cancelDownload', () {
    test('should cancel active download', () {});
    test('should clean up partial files', () {});
  });
});
```

### Mock Cleanup
```dart
setUp(() {
  mock = MockService();
});

tearDown(() {
  reset(mock);  // Reset mock state
  service.dispose();  // Clean up resources
});
```

### Avoid Flaky Tests
```dart
// Bad: Time-dependent
await Future.delayed(Duration(milliseconds: 100));

// Good: Use pumpAndSettle or explicit conditions
await tester.pumpAndSettle();
```

---

## 📚 Related Documentation

- [Build Guide](build-guide.md) - Building and CI/CD
- [Setup Guide](setup-guide.md) - Development environment
- [Architecture](../architecture/mobile-app-architecture.md) - Design patterns

---

**Last Updated:** October 8, 2025  
**Test Count:** 183 total (144 VM + 39 device)  
**Coverage:** ~70% overall
