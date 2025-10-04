# Pure Dart Implementation Guide

This guide provides a concrete implementation plan for migrating from Rust FFI to a pure Flutter/Dart architecture.

## Overview

The pure Dart approach eliminates FFI complexity by implementing all core functionality in Dart, using the excellent async/await primitives and Flutter ecosystem packages.

## Core Dependencies

```yaml
# pubspec.yaml
dependencies:
  # HTTP Client with advanced features
  dio: ^5.9.0
  
  # File system and storage
  path_provider: ^2.1.5
  
  # Compression support
  archive: ^3.6.0
  
  # Background processing
  flutter_isolate: ^2.0.4
  workmanager: ^0.5.2
  
  # Local database
  hive: ^2.2.3
  hive_flutter: ^1.1.0
  
  # State management
  provider: ^6.1.5
  
  # Utilities
  collection: ^1.18.0
  rxdart: ^0.28.0
```

## Architecture

### Layer Structure

```
┌─────────────────────────────────────────────────┐
│ Presentation Layer (Screens & Widgets)          │
├─────────────────────────────────────────────────┤
│ State Management (Provider/Riverpod)            │
├─────────────────────────────────────────────────┤
│ Service Layer (Business Logic)                  │
│  • ArchiveApiService                           │
│  • DownloadService                             │
│  • CompressionService                          │
│  • StorageService                              │
├─────────────────────────────────────────────────┤
│ Data Layer (Models & Repository)                │
│  • ArchiveMetadata                             │
│  • FileInfo                                    │
│  • DownloadTask                                │
├─────────────────────────────────────────────────┤
│ Infrastructure (Network & Storage)              │
│  • Dio HTTP Client                             │
│  • Hive Database                               │
│  • File System                                 │
└─────────────────────────────────────────────────┘
```

## Core Services Implementation

### 1. Archive API Service

```dart
// lib/services/archive_api_service.dart

import 'package:dio/dio.dart';
import '../models/archive_metadata.dart';

class ArchiveApiService {
  final Dio _dio;
  
  ArchiveApiService({Dio? dio}) 
    : _dio = dio ?? Dio(BaseOptions(
        baseUrl: 'https://archive.org',
        connectTimeout: Duration(seconds: 30),
        receiveTimeout: Duration(seconds: 30),
      ));

  /// Fetch metadata for an archive item
  Future<ArchiveMetadata> fetchMetadata(String identifier) async {
    try {
      final response = await _dio.get(
        '/metadata/$identifier',
        options: Options(
          headers: {
            'Accept': 'application/json',
          },
        ),
      );
      
      return ArchiveMetadata.fromJson(response.data);
    } on DioException catch (e) {
      throw _handleDioError(e);
    }
  }

  /// Search for archives
  Future<List<ArchiveMetadata>> search(String query, {
    int page = 1,
    int rows = 50,
  }) async {
    try {
      final response = await _dio.get(
        '/advancedsearch.php',
        queryParameters: {
          'q': query,
          'page': page,
          'rows': rows,
          'output': 'json',
        },
      );
      
      final docs = response.data['response']['docs'] as List;
      return docs.map((doc) => ArchiveMetadata.fromJson(doc)).toList();
    } on DioException catch (e) {
      throw _handleDioError(e);
    }
  }

  Exception _handleDioError(DioException e) {
    if (e.type == DioExceptionType.connectionTimeout ||
        e.type == DioExceptionType.receiveTimeout) {
      return TimeoutException('Request timeout');
    } else if (e.type == DioExceptionType.badResponse) {
      return HttpException('HTTP ${e.response?.statusCode}: ${e.message}');
    } else {
      return NetworkException('Network error: ${e.message}');
    }
  }
}
```

### 2. Download Service

```dart
// lib/services/download_service.dart

import 'dart:async';
import 'dart:io';
import 'dart:isolate';
import 'package:dio/dio.dart';
import '../models/download_task.dart';
import '../models/download_progress.dart';

class DownloadService {
  final Dio _dio;
  final Map<String, CancelToken> _cancelTokens = {};
  final StreamController<DownloadProgress> _progressController = 
    StreamController.broadcast();

  DownloadService({Dio? dio})
    : _dio = dio ?? Dio();

  Stream<DownloadProgress> get progressStream => _progressController.stream;

  /// Download a file with progress tracking and resume support
  Future<void> downloadFile(
    String url,
    String savePath, {
    String? taskId,
    Map<String, String>? headers,
  }) async {
    final id = taskId ?? url;
    final cancelToken = CancelToken();
    _cancelTokens[id] = cancelToken;

    final file = File(savePath);
    int downloadedBytes = 0;

    // Check if file exists for resume
    if (await file.exists()) {
      downloadedBytes = await file.length();
    }

    try {
      await _dio.download(
        url,
        savePath,
        cancelToken: cancelToken,
        options: Options(
          headers: {
            if (downloadedBytes > 0)
              'Range': 'bytes=$downloadedBytes-',
            ...?headers,
          },
          receiveTimeout: Duration(minutes: 30),
        ),
        onReceiveProgress: (received, total) {
          final progress = DownloadProgress(
            taskId: id,
            downloaded: downloadedBytes + received,
            total: total > 0 ? downloadedBytes + total : -1,
            speed: 0, // Calculate from time deltas
            status: DownloadStatus.downloading,
          );
          
          _progressController.add(progress);
        },
      );

      _progressController.add(DownloadProgress(
        taskId: id,
        downloaded: await file.length(),
        total: await file.length(),
        speed: 0,
        status: DownloadStatus.completed,
      ));
    } on DioException catch (e) {
      if (e.type == DioExceptionType.cancel) {
        _progressController.add(DownloadProgress(
          taskId: id,
          downloaded: downloadedBytes,
          total: -1,
          speed: 0,
          status: DownloadStatus.paused,
        ));
      } else {
        _progressController.add(DownloadProgress(
          taskId: id,
          downloaded: downloadedBytes,
          total: -1,
          speed: 0,
          status: DownloadStatus.failed,
          error: e.message,
        ));
        rethrow;
      }
    } finally {
      _cancelTokens.remove(id);
    }
  }

  /// Download multiple files concurrently
  Future<void> downloadMultiple(
    List<DownloadTask> tasks, {
    int maxConcurrent = 4,
  }) async {
    final queue = <Future>[];
    
    for (final task in tasks) {
      if (queue.length >= maxConcurrent) {
        await Future.any(queue);
        queue.removeWhere((future) => future.isCompleted);
      }
      
      final future = downloadFile(
        task.url,
        task.savePath,
        taskId: task.id,
        headers: task.headers,
      );
      
      queue.add(future);
    }
    
    await Future.wait(queue);
  }

  /// Pause a download
  void pauseDownload(String taskId) {
    _cancelTokens[taskId]?.cancel('Paused by user');
  }

  /// Cancel all downloads
  void cancelAll() {
    for (final token in _cancelTokens.values) {
      token.cancel('Cancelled by user');
    }
    _cancelTokens.clear();
  }

  void dispose() {
    cancelAll();
    _progressController.close();
  }
}
```

### 3. Compression Service

```dart
// lib/services/compression_service.dart

import 'dart:io';
import 'package:archive/archive.dart';
import 'package:path/path.dart' as path;

class CompressionService {
  /// Decompress a ZIP file
  Future<List<String>> decompressZip(
    String zipPath,
    String outputDir,
  ) async {
    final bytes = await File(zipPath).readAsBytes();
    final archive = ZipDecoder().decodeBytes(bytes);
    
    final extractedFiles = <String>[];
    
    for (final file in archive) {
      final filename = path.join(outputDir, file.name);
      
      if (file.isFile) {
        final outFile = File(filename);
        await outFile.create(recursive: true);
        await outFile.writeAsBytes(file.content as List<int>);
        extractedFiles.add(filename);
      } else {
        await Directory(filename).create(recursive: true);
      }
    }
    
    return extractedFiles;
  }

  /// Decompress a GZIP file
  Future<String> decompressGzip(
    String gzipPath,
    String outputPath,
  ) async {
    final bytes = await File(gzipPath).readAsBytes();
    final decompressed = GZipDecoder().decodeBytes(bytes);
    
    final outFile = File(outputPath);
    await outFile.writeAsBytes(decompressed);
    
    return outputPath;
  }

  /// Decompress a BZIP2 file
  Future<String> decompressBzip2(
    String bzip2Path,
    String outputPath,
  ) async {
    final bytes = await File(bzip2Path).readAsBytes();
    final decompressed = BZip2Decoder().decodeBytes(bytes);
    
    final outFile = File(outputPath);
    await outFile.writeAsBytes(decompressed);
    
    return outputPath;
  }

  /// Decompress a TAR file
  Future<List<String>> decompressTar(
    String tarPath,
    String outputDir,
  ) async {
    final bytes = await File(tarPath).readAsBytes();
    final archive = TarDecoder().decodeBytes(bytes);
    
    final extractedFiles = <String>[];
    
    for (final file in archive) {
      final filename = path.join(outputDir, file.name);
      
      if (file.isFile) {
        final outFile = File(filename);
        await outFile.create(recursive: true);
        await outFile.writeAsBytes(file.content as List<int>);
        extractedFiles.add(filename);
      } else {
        await Directory(filename).create(recursive: true);
      }
    }
    
    return extractedFiles;
  }

  /// Auto-detect and decompress based on file extension
  Future<dynamic> decompressAuto(
    String filePath,
    String outputDir,
  ) async {
    final ext = path.extension(filePath).toLowerCase();
    
    switch (ext) {
      case '.zip':
        return await decompressZip(filePath, outputDir);
      case '.gz':
      case '.gzip':
        final outputPath = path.join(
          outputDir,
          path.basenameWithoutExtension(filePath),
        );
        return await decompressGzip(filePath, outputPath);
      case '.bz2':
      case '.bzip2':
        final outputPath = path.join(
          outputDir,
          path.basenameWithoutExtension(filePath),
        );
        return await decompressBzip2(filePath, outputPath);
      case '.tar':
        return await decompressTar(filePath, outputDir);
      default:
        throw UnsupportedError('Unsupported compression format: $ext');
    }
  }
}
```

### 4. Storage Service

```dart
// lib/services/storage_service.dart

import 'dart:io';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:path_provider/path_provider.dart';
import '../models/download_history.dart';

class StorageService {
  static const _historyBox = 'download_history';
  static const _settingsBox = 'settings';
  
  late Box<DownloadHistory> _history;
  late Box _settings;

  Future<void> initialize() async {
    await Hive.initFlutter();
    
    // Register adapters
    Hive.registerAdapter(DownloadHistoryAdapter());
    
    // Open boxes
    _history = await Hive.openBox<DownloadHistory>(_historyBox);
    _settings = await Hive.openBox(_settingsBox);
  }

  /// Save download to history
  Future<void> saveDownload(DownloadHistory download) async {
    await _history.put(download.id, download);
  }

  /// Get download history
  List<DownloadHistory> getHistory() {
    return _history.values.toList()
      ..sort((a, b) => b.timestamp.compareTo(a.timestamp));
  }

  /// Clear download history
  Future<void> clearHistory() async {
    await _history.clear();
  }

  /// Get setting value
  T? getSetting<T>(String key, {T? defaultValue}) {
    return _settings.get(key, defaultValue: defaultValue) as T?;
  }

  /// Set setting value
  Future<void> setSetting<T>(String key, T value) async {
    await _settings.put(key, value);
  }

  /// Get downloads directory
  Future<String> getDownloadsDirectory() async {
    if (Platform.isAndroid) {
      // Use external storage on Android
      final dir = await getExternalStorageDirectory();
      return dir?.path ?? '/storage/emulated/0/Download';
    } else {
      final dir = await getApplicationDocumentsDirectory();
      return dir.path;
    }
  }

  void dispose() {
    _history.close();
    _settings.close();
  }
}
```

## Data Models

### Archive Metadata

```dart
// lib/models/archive_metadata.dart

import 'package:json_annotation/json_annotation.dart';

part 'archive_metadata.g.dart';

@JsonSerializable()
class ArchiveMetadata {
  final String identifier;
  final String title;
  final String? description;
  final List<ArchiveFile> files;
  final Map<String, dynamic>? metadata;

  ArchiveMetadata({
    required this.identifier,
    required this.title,
    this.description,
    required this.files,
    this.metadata,
  });

  factory ArchiveMetadata.fromJson(Map<String, dynamic> json) =>
      _$ArchiveMetadataFromJson(json);

  Map<String, dynamic> toJson() => _$ArchiveMetadataToJson(this);
}

@JsonSerializable()
class ArchiveFile {
  final String name;
  final int? size;
  final String? format;
  final String? md5;
  final String? sha1;

  ArchiveFile({
    required this.name,
    this.size,
    this.format,
    this.md5,
    this.sha1,
  });

  factory ArchiveFile.fromJson(Map<String, dynamic> json) =>
      _$ArchiveFileFromJson(json);

  Map<String, dynamic> toJson() => _$ArchiveFileToJson(this);
}
```

### Download Task

```dart
// lib/models/download_task.dart

import 'package:uuid/uuid.dart';

class DownloadTask {
  final String id;
  final String url;
  final String savePath;
  final Map<String, String>? headers;
  final DateTime createdAt;

  DownloadTask({
    String? id,
    required this.url,
    required this.savePath,
    this.headers,
    DateTime? createdAt,
  })  : id = id ?? const Uuid().v4(),
        createdAt = createdAt ?? DateTime.now();
}
```

### Download Progress

```dart
// lib/models/download_progress.dart

enum DownloadStatus {
  pending,
  downloading,
  paused,
  completed,
  failed,
}

class DownloadProgress {
  final String taskId;
  final int downloaded;
  final int total;
  final double speed;
  final DownloadStatus status;
  final String? error;

  DownloadProgress({
    required this.taskId,
    required this.downloaded,
    required this.total,
    required this.speed,
    required this.status,
    this.error,
  });

  double get progress {
    if (total <= 0) return 0.0;
    return downloaded / total;
  }

  String get progressText {
    if (total <= 0) return '${_formatBytes(downloaded)} / ?';
    return '${_formatBytes(downloaded)} / ${_formatBytes(total)}';
  }

  String get speedText => '${_formatBytes(speed.toInt())}/s';

  String _formatBytes(int bytes) {
    if (bytes < 1024) return '$bytes B';
    if (bytes < 1024 * 1024) return '${(bytes / 1024).toStringAsFixed(1)} KB';
    if (bytes < 1024 * 1024 * 1024) {
      return '${(bytes / (1024 * 1024)).toStringAsFixed(1)} MB';
    }
    return '${(bytes / (1024 * 1024 * 1024)).toStringAsFixed(1)} GB';
  }
}
```

## State Management with Provider

```dart
// lib/providers/download_provider.dart

import 'package:flutter/foundation.dart';
import '../services/download_service.dart';
import '../models/download_progress.dart';
import '../models/download_task.dart';

class DownloadProvider extends ChangeNotifier {
  final DownloadService _downloadService;
  final Map<String, DownloadProgress> _downloads = {};

  DownloadProvider(this._downloadService) {
    _downloadService.progressStream.listen((progress) {
      _downloads[progress.taskId] = progress;
      notifyListeners();
    });
  }

  Map<String, DownloadProgress> get downloads => Map.unmodifiable(_downloads);

  Future<void> startDownload(DownloadTask task) async {
    await _downloadService.downloadFile(
      task.url,
      task.savePath,
      taskId: task.id,
      headers: task.headers,
    );
  }

  Future<void> startMultipleDownloads(
    List<DownloadTask> tasks, {
    int maxConcurrent = 4,
  }) async {
    await _downloadService.downloadMultiple(
      tasks,
      maxConcurrent: maxConcurrent,
    );
  }

  void pauseDownload(String taskId) {
    _downloadService.pauseDownload(taskId);
  }

  void cancelAllDownloads() {
    _downloadService.cancelAll();
    _downloads.clear();
    notifyListeners();
  }

  @override
  void dispose() {
    _downloadService.dispose();
    super.dispose();
  }
}
```

## Usage Example

```dart
// lib/main.dart

import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'services/archive_api_service.dart';
import 'services/download_service.dart';
import 'services/storage_service.dart';
import 'providers/download_provider.dart';
import 'screens/home_screen.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  
  // Initialize services
  final storageService = StorageService();
  await storageService.initialize();
  
  final apiService = ArchiveApiService();
  final downloadService = DownloadService();
  
  runApp(
    MultiProvider(
      providers: [
        Provider.value(value: apiService),
        Provider.value(value: storageService),
        ChangeNotifierProvider(
          create: (_) => DownloadProvider(downloadService),
        ),
      ],
      child: const MyApp(),
    ),
  );
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Internet Archive Helper',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.blue),
        useMaterial3: true,
      ),
      home: const HomeScreen(),
    );
  }
}
```

## Testing

```dart
// test/services/download_service_test.dart

import 'package:flutter_test/flutter_test.dart';
import 'package:mockito/mockito.dart';
import 'package:dio/dio.dart';
import 'package:ia_get/services/download_service.dart';

void main() {
  group('DownloadService', () {
    late DownloadService downloadService;
    late MockDio mockDio;

    setUp(() {
      mockDio = MockDio();
      downloadService = DownloadService(dio: mockDio);
    });

    test('downloads file successfully', () async {
      // Arrange
      when(mockDio.download(
        any,
        any,
        cancelToken: anyNamed('cancelToken'),
        options: anyNamed('options'),
        onReceiveProgress: anyNamed('onReceiveProgress'),
      )).thenAnswer((_) async => Response(
        requestOptions: RequestOptions(path: ''),
        statusCode: 200,
      ));

      // Act
      await downloadService.downloadFile(
        'https://example.com/file.zip',
        '/tmp/file.zip',
      );

      // Assert
      verify(mockDio.download(any, any,
        cancelToken: anyNamed('cancelToken'),
        options: anyNamed('options'),
        onReceiveProgress: anyNamed('onReceiveProgress'),
      )).called(1);
    });

    test('handles download cancellation', () async {
      // Arrange
      when(mockDio.download(
        any,
        any,
        cancelToken: anyNamed('cancelToken'),
        options: anyNamed('options'),
        onReceiveProgress: anyNamed('onReceiveProgress'),
      )).thenThrow(DioException(
        requestOptions: RequestOptions(path: ''),
        type: DioExceptionType.cancel,
      ));

      // Act & Assert
      expect(
        () => downloadService.downloadFile(
          'https://example.com/file.zip',
          '/tmp/file.zip',
        ),
        throwsA(isA<DioException>()),
      );
    });
  });
}
```

## Migration Plan

### Step 1: Parallel Implementation (Week 1-2)
- [ ] Create new pure Dart services alongside existing FFI
- [ ] Implement basic metadata fetching
- [ ] Add simple download functionality
- [ ] Build minimal UI for testing

### Step 2: Feature Parity (Week 3-4)
- [ ] Implement all core features in Dart
- [ ] Add compression support
- [ ] Implement download resume
- [ ] Add concurrent downloads

### Step 3: Testing & Validation (Week 5)
- [ ] Comprehensive unit tests
- [ ] Integration tests
- [ ] Performance testing
- [ ] User acceptance testing

### Step 4: Cutover (Week 6)
- [ ] Switch default implementation to pure Dart
- [ ] Deprecate FFI code
- [ ] Update documentation
- [ ] Release new version

## Performance Considerations

### Dart Performance
- Dart is JIT compiled in debug mode (fast iteration)
- AOT compiled in release mode (native performance)
- Excellent async/await for I/O operations
- Isolates for CPU-intensive tasks

### Optimization Tips
- Use Isolates for heavy computation
- Leverage Dio's connection pooling
- Implement efficient caching
- Use streams for progress updates
- Profile with Flutter DevTools

## Conclusion

The pure Dart implementation provides:
- ✅ Simpler architecture
- ✅ Better debugging experience
- ✅ Faster development cycle
- ✅ No FFI complexity
- ✅ Single source of truth

This approach is ideal for mobile-first development where Flutter/Dart is the primary platform.
