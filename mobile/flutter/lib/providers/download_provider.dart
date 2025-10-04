/// Download State Management Provider
///
/// Manages all download state in Dart, using the simplified FFI interface.
/// This eliminates race conditions by having a single source of truth in Dart.

import 'package:flutter/foundation.dart';
import '../models/archive_metadata.dart';
import '../models/download_progress.dart';
import '../services/ia_get_simple_service.dart';

/// Download state machine for clear state transitions
enum DownloadStatus {
  idle,
  fetchingMetadata,
  downloading,
  validating,
  extracting,
  complete,
  error,
  cancelled,
}

/// Extension to convert DownloadStatus to string for backward compatibility
extension DownloadStatusExtension on DownloadStatus {
  String get value {
    switch (this) {
      case DownloadStatus.idle:
        return 'idle';
      case DownloadStatus.fetchingMetadata:
        return 'fetching_metadata';
      case DownloadStatus.downloading:
        return 'downloading';
      case DownloadStatus.validating:
        return 'validating';
      case DownloadStatus.extracting:
        return 'extracting';
      case DownloadStatus.complete:
        return 'complete';
      case DownloadStatus.error:
        return 'error';
      case DownloadStatus.cancelled:
        return 'cancelled';
    }
  }
  
  /// Check if download is in progress
  bool get isActive {
    return this == DownloadStatus.fetchingMetadata ||
           this == DownloadStatus.downloading ||
           this == DownloadStatus.validating ||
           this == DownloadStatus.extracting;
  }
  
  /// Check if download has finished (success or failure)
  bool get isFinished {
    return this == DownloadStatus.complete ||
           this == DownloadStatus.error ||
           this == DownloadStatus.cancelled;
  }
}

/// Download state for a single item
class DownloadState {
  final String identifier;
  final ArchiveMetadata? metadata;
  final Map<String, DownloadProgress> fileProgress;
  final DownloadStatus downloadStatus;
  final String? error;
  final DateTime? startTime;
  final DateTime? endTime;

  DownloadState({
    required this.identifier,
    this.metadata,
    Map<String, DownloadProgress>? fileProgress,
    this.downloadStatus = DownloadStatus.idle,
    this.error,
    this.startTime,
    this.endTime,
  }) : fileProgress = fileProgress ?? {};
  
  /// Get status as string for backward compatibility
  String get status => downloadStatus.value;

  DownloadState copyWith({
    String? identifier,
    ArchiveMetadata? metadata,
    Map<String, DownloadProgress>? fileProgress,
    DownloadStatus? downloadStatus,
    String? status,  // Deprecated, use downloadStatus
    String? error,
    DateTime? startTime,
    DateTime? endTime,
  }) {
    return DownloadState(
      identifier: identifier ?? this.identifier,
      metadata: metadata ?? this.metadata,
      fileProgress: fileProgress ?? this.fileProgress,
      downloadStatus: downloadStatus ?? 
                      (status != null ? _parseStatus(status) : this.downloadStatus),
      error: error ?? this.error,
      startTime: startTime ?? this.startTime,
      endTime: endTime ?? this.endTime,
    );
  }
  
  /// Parse status string to enum for backward compatibility
  static DownloadStatus _parseStatus(String status) {
    switch (status) {
      case 'fetching_metadata':
        return DownloadStatus.fetchingMetadata;
      case 'downloading':
        return DownloadStatus.downloading;
      case 'validating':
        return DownloadStatus.validating;
      case 'extracting':
        return DownloadStatus.extracting;
      case 'complete':
        return DownloadStatus.complete;
      case 'error':
        return DownloadStatus.error;
      case 'cancelled':
        return DownloadStatus.cancelled;
      default:
        return DownloadStatus.idle;
    }
  }

  double get overallProgress {
    if (fileProgress.isEmpty) return 0.0;
    
    final totalPercentage = fileProgress.values
        .map((p) => p.percentage)
        .reduce((a, b) => a + b);
    
    return totalPercentage / fileProgress.length;
  }

  int get totalDownloaded {
    return fileProgress.values
        .map((p) => p.downloaded)
        .fold(0, (a, b) => a + b);
  }

  int get totalSize {
    return fileProgress.values
        .map((p) => p.total)
        .fold(0, (a, b) => a + b);
  }
}

/// Download Provider - manages all download state in Dart
///
/// This is the single source of truth for download state, eliminating
/// race conditions between Rust and Dart.
class DownloadProvider extends ChangeNotifier {
  final IaGetSimpleService _service = IaGetSimpleService();
  
  // State management - all in Dart!
  final Map<String, DownloadState> _downloads = {};
  final List<String> _downloadHistory = [];
  
  // Enhanced: Metadata caching for better performance
  final Map<String, ArchiveMetadata> _metadataCache = {};
  
  // Enhanced: Concurrent download configuration
  int maxConcurrentDownloads = 3;
  int _activeDownloads = 0;

  /// Get all downloads
  Map<String, DownloadState> get downloads => Map.unmodifiable(_downloads);

  /// Get download history
  List<String> get downloadHistory => List.unmodifiable(_downloadHistory);
  
  /// Get active download count
  int get activeDownloadCount => _activeDownloads;

  /// Get specific download state
  DownloadState? getDownload(String identifier) {
    return _downloads[identifier];
  }
  
  /// Get cached metadata if available
  ArchiveMetadata? getCachedMetadata(String identifier) {
    return _metadataCache[identifier];
  }
  
  /// Clear metadata cache
  void clearMetadataCache() {
    _metadataCache.clear();
    notifyListeners();
  }

  /// Start downloading an archive
  Future<void> startDownload(
    String identifier, {
    String? outputDir,
    List<String>? fileFilters,
  }) async {
    if (_downloads.containsKey(identifier)) {
      if (_downloads[identifier]!.downloadStatus.isActive) {
        throw Exception('Download already in progress for $identifier');
      }
    }

    // Initialize download state
    _downloads[identifier] = DownloadState(
      identifier: identifier,
      downloadStatus: DownloadStatus.fetchingMetadata,
      startTime: DateTime.now(),
    );
    notifyListeners();

    try {
      // Fetch metadata with caching
      ArchiveMetadata metadata;
      if (_metadataCache.containsKey(identifier)) {
        // Use cached metadata for better performance
        metadata = _metadataCache[identifier]!;
      } else {
        // Fetch from network and cache
        metadata = await _service.fetchMetadata(identifier);
        _metadataCache[identifier] = metadata;
      }
      
      _downloads[identifier] = _downloads[identifier]!.copyWith(
        metadata: metadata,
        downloadStatus: DownloadStatus.downloading,
      );
      notifyListeners();

      // Enhanced: Improved file filtering with multiple patterns
      var filesToDownload = metadata.files;
      if (fileFilters != null && fileFilters.isNotEmpty) {
        filesToDownload = filesToDownload.where((file) {
          final fileName = file.name.toLowerCase();
          return fileFilters.any((filter) {
            final filterLower = filter.toLowerCase();
            // Support both exact contains and wildcard patterns
            if (filterLower.contains('*')) {
              // Simple wildcard matching
              final pattern = filterLower.replaceAll('*', '.*');
              return RegExp(pattern).hasMatch(fileName);
            } else {
              // Exact substring matching
              return fileName.contains(filterLower);
            }
          });
        }).toList();
      }

      if (filesToDownload.isEmpty) {
        throw Exception('No files to download after filtering');
      }

      // Download each file
      final downloadDir = outputDir ?? '/sdcard/Download/ia-get/$identifier';
      
      // Track active downloads for concurrency control
      _activeDownloads++;
      
      for (final file in filesToDownload) {
        final url = 'https://archive.org/download/$identifier/${file.name}';
        final outputPath = '$downloadDir/${file.name}';

        // Initialize progress for this file
        final fileProgress = Map<String, DownloadProgress>.from(
          _downloads[identifier]!.fileProgress,
        );
        fileProgress[file.name] = DownloadProgress(
          downloaded: 0,
          total: file.size ?? 0,
          percentage: 0.0,
          status: 'starting',
        );
        
        _downloads[identifier] = _downloads[identifier]!.copyWith(
          fileProgress: fileProgress,
        );
        notifyListeners();

        // Download with progress tracking
        await _service.downloadFile(
          url,
          outputPath,
          onProgress: (downloaded, total) {
            final updatedProgress = Map<String, DownloadProgress>.from(
              _downloads[identifier]!.fileProgress,
            );
            updatedProgress[file.name] = DownloadProgress(
              downloaded: downloaded,
              total: total,
              percentage: total > 0 ? (downloaded / total) * 100 : 0.0,
              status: 'downloading',
            );
            
            _downloads[identifier] = _downloads[identifier]!.copyWith(
              fileProgress: updatedProgress,
            );
            notifyListeners();
          },
        );

        // Mark file as complete
        final updatedProgress = Map<String, DownloadProgress>.from(
          _downloads[identifier]!.fileProgress,
        );
        updatedProgress[file.name] = updatedProgress[file.name]!.copyWith(
          percentage: 100.0,
          status: 'complete',
        );
        
        _downloads[identifier] = _downloads[identifier]!.copyWith(
          fileProgress: updatedProgress,
        );
        notifyListeners();

        // Validate checksum if available
        if (file.md5 != null && file.md5!.isNotEmpty) {
          try {
            final isValid = await _service.validateChecksum(
              outputPath,
              file.md5!,
              hashType: 'md5',
            );

            if (!isValid) {
              throw Exception('Checksum validation failed for ${file.name}');
            }

            if (kDebugMode) {
              print('Checksum validated for ${file.name}');
            }
          } catch (e) {
            if (kDebugMode) {
              print('Warning: Checksum validation failed: $e');
            }
          }
        }

        // Decompress if it's an archive
        if (_isArchive(file.name)) {
          try {
            final extractedFiles = await _service.decompressFile(
              outputPath,
              downloadDir,
            );
            
            if (kDebugMode) {
              print('Extracted ${extractedFiles.length} files from ${file.name}');
            }
          } catch (e) {
            if (kDebugMode) {
              print('Warning: Failed to decompress ${file.name}: $e');
            }
          }
        }
      }

      // Mark download as complete
      _downloads[identifier] = _downloads[identifier]!.copyWith(
        downloadStatus: DownloadStatus.complete,
        endTime: DateTime.now(),
      );
      
      // Add to history
      if (!_downloadHistory.contains(identifier)) {
        _downloadHistory.insert(0, identifier);
        if (_downloadHistory.length > 100) {
          _downloadHistory.removeLast();
        }
      }
      
      notifyListeners();
    } catch (e, stackTrace) {
      if (kDebugMode) {
        print('Download failed: $e');
        print('Stack trace: $stackTrace');
      }

      // Enhanced: More specific error messages
      String errorMessage = e.toString();
      if (errorMessage.contains('network') || errorMessage.contains('HTTP')) {
        errorMessage = 'Network error: Please check your internet connection';
      } else if (errorMessage.contains('permission') || errorMessage.contains('denied')) {
        errorMessage = 'Permission error: Cannot write to destination';
      } else if (errorMessage.contains('space') || errorMessage.contains('full')) {
        errorMessage = 'Storage error: Insufficient disk space';
      }

      _downloads[identifier] = _downloads[identifier]!.copyWith(
        downloadStatus: DownloadStatus.error,
        error: errorMessage,
        endTime: DateTime.now(),
      );
      notifyListeners();

      rethrow;
    } finally {
      // Ensure active download count is decremented
      if (_activeDownloads > 0) {
        _activeDownloads--;
      }
    }
  }

  /// Cancel a download
  Future<void> cancelDownload(String identifier) async {
    if (!_downloads.containsKey(identifier)) {
      return;
    }

    // Note: In a full implementation, you'd need to signal the Rust side
    // to cancel the download. For now, we just update the state.
    _downloads[identifier] = _downloads[identifier]!.copyWith(
      downloadStatus: DownloadStatus.cancelled,
      endTime: DateTime.now(),
    );
    notifyListeners();
  }

  /// Clear a download from state
  void clearDownload(String identifier) {
    _downloads.remove(identifier);
    notifyListeners();
  }

  /// Clear all completed downloads
  void clearCompletedDownloads() {
    _downloads.removeWhere((_, state) => state.status == 'complete');
    notifyListeners();
  }

  /// Clear download history
  void clearHistory() {
    _downloadHistory.clear();
    notifyListeners();
  }

  /// Check if file is an archive
  bool _isArchive(String filename) {
    final lowercaseName = filename.toLowerCase();
    return lowercaseName.endsWith('.zip') ||
        lowercaseName.endsWith('.tar') ||
        lowercaseName.endsWith('.tar.gz') ||
        lowercaseName.endsWith('.tgz') ||
        lowercaseName.endsWith('.tar.bz2') ||
        lowercaseName.endsWith('.tbz2') ||
        lowercaseName.endsWith('.tar.xz') ||
        lowercaseName.endsWith('.txz') ||
        lowercaseName.endsWith('.gz') ||
        lowercaseName.endsWith('.bz2') ||
        lowercaseName.endsWith('.xz');
  }
}

/// Extension to add copyWith to DownloadProgress
extension DownloadProgressCopyWith on DownloadProgress {
  DownloadProgress copyWith({
    int? downloaded,
    int? total,
    double? percentage,
    String? status,
    String? error,
  }) {
    return DownloadProgress(
      downloaded: downloaded ?? this.downloaded,
      total: total ?? this.total,
      percentage: percentage ?? this.percentage,
      status: status ?? this.status,
      error: error ?? this.error,
    );
  }
}
