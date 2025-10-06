import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:percent_indicator/percent_indicator.dart';
import 'package:open_file/open_file.dart';
import '../providers/download_provider.dart';
import '../services/background_download_service.dart';
import '../models/download_progress.dart' as progress_model;
import '../utils/file_utils.dart';
import '../utils/permission_utils.dart';

class DownloadScreen extends StatefulWidget {
  const DownloadScreen({super.key, this.useBackground = false});

  /// When true, show downloads from BackgroundDownloadService instead of DownloadProvider
  final bool useBackground;

  /// Route name for navigation tracking
  static const routeName = '/downloads';

  @override
  State<DownloadScreen> createState() => _DownloadScreenState();
}

class _DownloadScreenState extends State<DownloadScreen> {
  @override
  Widget build(BuildContext context) {
    // Show background service downloads or provider downloads based on flag
    if (widget.useBackground) {
      return _buildBackgroundView();
    }
    return Consumer<DownloadProvider>(
      builder: (context, downloadProvider, child) {
        final downloads = downloadProvider.downloads;
        final activeDownloads = downloads.values
            .where((d) => d.downloadStatus.isActive)
            .toList();
        final completedDownloads = downloads.values
            .where((d) => d.downloadStatus == DownloadStatus.complete)
            .toList();

        return PopScope(
          canPop: true,
          child: Scaffold(
            appBar: AppBar(
              title: Row(
                children: [
                  const Text('Downloads'),
                  if (downloadProvider.activeDownloadCount > 0 ||
                      downloadProvider.queuedDownloadCount > 0)
                    Padding(
                      padding: const EdgeInsets.only(left: 8),
                      child: Text(
                        '(${downloadProvider.activeDownloadCount} active'
                        '${downloadProvider.queuedDownloadCount > 0 ? ', ${downloadProvider.queuedDownloadCount} queued' : ''})',
                        style: Theme.of(context).textTheme.bodySmall?.copyWith(
                          color: Colors.grey.shade300,
                        ),
                      ),
                    ),
                ],
              ),
              actions: [
                if (downloads.isNotEmpty)
                  IconButton(
                    icon: const Icon(Icons.clear_all),
                    onPressed: () => _clearAllDownloads(downloadProvider),
                  ),
              ],
            ),
            body: downloads.isEmpty
                ? const Center(
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Icon(Icons.download_done, size: 64, color: Colors.grey),
                        SizedBox(height: 16),
                        Text(
                          'No downloads yet',
                          style: TextStyle(fontSize: 16, color: Colors.grey),
                        ),
                        SizedBox(height: 8),
                        Text(
                          'Start downloading files from the main screen',
                          style: TextStyle(fontSize: 14, color: Colors.grey),
                        ),
                      ],
                    ),
                  )
                : ListView(
                    padding: const EdgeInsets.all(16),
                    children: [
                      if (activeDownloads.isNotEmpty) ...[
                        const Text(
                          'Active Downloads',
                          style: TextStyle(
                            fontSize: 18,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                        const SizedBox(height: 8),
                        ...activeDownloads.map(
                          (state) =>
                              _buildActiveDownloadCard(state, downloadProvider),
                        ),
                        const SizedBox(height: 24),
                      ],
                      if (completedDownloads.isNotEmpty) ...[
                        const Text(
                          'Completed Downloads',
                          style: TextStyle(
                            fontSize: 18,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                        const SizedBox(height: 8),
                        ...completedDownloads.map(
                          (state) => _buildCompletedDownloadCard(
                            state,
                            downloadProvider,
                          ),
                        ),
                      ],
                    ],
                  ),
          ),
        );
      },
    );
  }

  Widget _buildBackgroundView() {
    return Consumer<BackgroundDownloadService>(
      builder: (context, bgService, child) {
        final active = bgService.activeDownloads.values
            .where(
              (d) =>
                  d.status == progress_model.DownloadStatus.downloading ||
                  d.status == progress_model.DownloadStatus.queued ||
                  d.status == progress_model.DownloadStatus.paused,
            )
            .toList();
        final completed = bgService.completedDownloads.values.toList();

        return Scaffold(
          appBar: AppBar(
            title: const Text('Downloads'),
            actions: [
              if (completed.isNotEmpty)
                IconButton(
                  icon: const Icon(Icons.clear_all),
                  onPressed: () => bgService.clearCompletedDownloads(),
                ),
            ],
          ),
          body: bgService.totalDownloadCount == 0
              ? const Center(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Icon(Icons.download_done, size: 64, color: Colors.grey),
                      SizedBox(height: 16),
                      Text(
                        'No downloads yet',
                        style: TextStyle(fontSize: 16, color: Colors.grey),
                      ),
                      SizedBox(height: 8),
                      Text(
                        'Start downloading files from the main screen',
                        style: TextStyle(fontSize: 14, color: Colors.grey),
                      ),
                    ],
                  ),
                )
              : ListView(
                  padding: const EdgeInsets.all(16),
                  children: [
                    if (active.isNotEmpty) ...[
                      const Text(
                        'Active Downloads',
                        style: TextStyle(
                          fontSize: 18,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      const SizedBox(height: 8),
                      ...active.map(
                        (p) =>
                            _buildActiveDownloadCardForProgress(p, bgService),
                      ),
                      const SizedBox(height: 24),
                    ],
                    if (completed.isNotEmpty) ...[
                      const Text(
                        'Completed Downloads',
                        style: TextStyle(
                          fontSize: 18,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      const SizedBox(height: 8),
                      ...completed.map(
                        (p) => _buildCompletedDownloadCardForProgress(
                          p,
                          bgService,
                        ),
                      ),
                    ],
                  ],
                ),
        );
      },
    );
  }

  Widget _buildActiveDownloadCardForProgress(
    progress_model.DownloadProgress p,
    BackgroundDownloadService svc,
  ) {
    final prog = (p.progress ?? 0.0).clamp(0.0, 1.0);
    return Card(
      margin: const EdgeInsets.only(bottom: 12),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Expanded(
                  child: Text(
                    p.identifier,
                    style: const TextStyle(
                      fontWeight: FontWeight.w500,
                      fontSize: 16,
                    ),
                    overflow: TextOverflow.ellipsis,
                  ),
                ),
                IconButton(
                  icon: const Icon(Icons.stop),
                  tooltip: 'Cancel download',
                  onPressed: () => svc.cancelDownload(p.downloadId),
                ),
              ],
            ),
            const SizedBox(height: 8),
            Text(
              'Downloading ${p.completedFiles ?? 0}/${p.totalFiles} files',
              style: TextStyle(color: Colors.grey.shade600),
            ),
            const SizedBox(height: 12),
            LinearPercentIndicator(
              percent: prog,
              backgroundColor: Colors.grey.shade300,
              progressColor: Theme.of(context).primaryColor,
              lineHeight: 8,
              barRadius: const Radius.circular(4),
            ),
            const SizedBox(height: 8),
            Text(
              '${(prog * 100).toStringAsFixed(1)}%',
              style: const TextStyle(fontWeight: FontWeight.w500),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildCompletedDownloadCardForProgress(
    progress_model.DownloadProgress p,
    BackgroundDownloadService svc,
  ) {
    return Card(
      margin: const EdgeInsets.only(bottom: 12),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Row(
          children: [
            const Icon(Icons.check_circle, color: Colors.green, size: 24),
            const SizedBox(width: 12),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    p.identifier,
                    style: const TextStyle(fontWeight: FontWeight.w500),
                    overflow: TextOverflow.ellipsis,
                  ),
                  Text(
                    '${p.totalFiles} files • ${FileUtils.formatBytes(p.totalBytes ?? 0)}',
                    style: TextStyle(fontSize: 12, color: Colors.grey.shade600),
                  ),
                ],
              ),
            ),
            IconButton(
              icon: const Icon(Icons.folder_open),
              tooltip: 'Open download folder',
              onPressed: () => _openDownloadFolderForProgress(p.identifier),
            ),
          ],
        ),
      ),
    );
  }

  void _openDownloadFolderForProgress(String identifier) async {
    // Check if we have permission to access folders
    final hasPermission = await PermissionUtils.hasManageStoragePermission();

    if (!hasPermission) {
      if (!mounted) return;

      // Request permission with explanation
      final granted = await PermissionUtils.requestManageStoragePermission(
        context,
      );

      if (!granted) {
        // User denied permission
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(
              content: Text(
                'Storage access permission is required to open folders',
              ),
              duration: Duration(seconds: 3),
            ),
          );
        }
        return;
      }
    }

    final downloadPath = '/storage/emulated/0/Download/ia-get/$identifier';
    try {
      final result = await OpenFile.open(downloadPath);
      if (mounted && result.type != ResultType.done) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Could not open folder: ${result.message}')),
        );
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(
          context,
        ).showSnackBar(SnackBar(content: Text('Error opening folder: $e')));
      }
    }
  }

  Widget _buildActiveDownloadCard(
    DownloadState downloadState,
    DownloadProvider provider,
  ) {
    final identifier = downloadState.identifier;
    final overallProgress =
        downloadState.overallProgress / 100.0; // Convert to 0-1 range

    return Card(
      margin: const EdgeInsets.only(bottom: 12),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Expanded(
                  child: Text(
                    identifier,
                    style: const TextStyle(
                      fontWeight: FontWeight.w500,
                      fontSize: 16,
                    ),
                    overflow: TextOverflow.ellipsis,
                  ),
                ),
                IconButton(
                  icon: const Icon(Icons.stop),
                  tooltip: 'Cancel download',
                  onPressed: () => _cancelDownload(identifier, provider),
                ),
              ],
            ),
            const SizedBox(height: 8),
            Text(
              downloadState.downloadStatus == DownloadStatus.fetchingMetadata
                  ? 'Fetching metadata...'
                  : 'Downloading ${downloadState.fileProgress.length} files',
              style: TextStyle(color: Colors.grey.shade600),
            ),
            const SizedBox(height: 12),
            LinearPercentIndicator(
              percent: overallProgress,
              backgroundColor: Colors.grey.shade300,
              progressColor: Theme.of(context).primaryColor,
              lineHeight: 8,
              barRadius: const Radius.circular(4),
            ),
            const SizedBox(height: 8),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text(
                  '${(overallProgress * 100).toStringAsFixed(1)}%',
                  style: const TextStyle(fontWeight: FontWeight.w500),
                ),
                Text(
                  '${FileUtils.formatSize(downloadState.totalDownloaded)} / ${FileUtils.formatSize(downloadState.totalSize)}',
                  style: TextStyle(color: Colors.grey.shade600),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildCompletedDownloadCard(
    DownloadState downloadState,
    DownloadProvider provider,
  ) {
    final identifier = downloadState.identifier;
    final fileCount = downloadState.metadata?.files.length ?? 0;

    return Card(
      margin: const EdgeInsets.only(bottom: 12),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Row(
          children: [
            const Icon(Icons.check_circle, color: Colors.green, size: 24),
            const SizedBox(width: 12),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    identifier,
                    style: const TextStyle(fontWeight: FontWeight.w500),
                    overflow: TextOverflow.ellipsis,
                  ),
                  Text(
                    '$fileCount files • ${FileUtils.formatSize(downloadState.totalSize)}',
                    style: TextStyle(fontSize: 12, color: Colors.grey.shade600),
                  ),
                ],
              ),
            ),
            IconButton(
              icon: const Icon(Icons.folder_open),
              tooltip: 'Open download folder',
              onPressed: () => _openDownloadFolder(identifier),
            ),
            IconButton(
              icon: const Icon(Icons.delete),
              tooltip: 'Clear from list',
              onPressed: () => provider.clearDownload(identifier),
            ),
          ],
        ),
      ),
    );
  }

  void _cancelDownload(String identifier, DownloadProvider provider) async {
    try {
      await provider.cancelDownload(identifier);
      if (mounted) {
        ScaffoldMessenger.of(
          context,
        ).showSnackBar(const SnackBar(content: Text('Download cancelled')));
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(
          context,
        ).showSnackBar(SnackBar(content: Text('Failed to cancel: $e')));
      }
    }
  }

  void _clearAllDownloads(DownloadProvider provider) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Clear All Downloads'),
        content: const Text('Are you sure you want to clear all downloads?'),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () {
              provider.clearCompletedDownloads();
              Navigator.pop(context);
            },
            child: const Text('Clear'),
          ),
        ],
      ),
    );
  }

  void _openDownloadFolder(String identifier) async {
    // Check if we have permission to access folders
    final hasPermission = await PermissionUtils.hasManageStoragePermission();

    if (!hasPermission) {
      if (!mounted) return;

      // Request permission with explanation
      final granted = await PermissionUtils.requestManageStoragePermission(
        context,
      );

      if (!granted) {
        // User denied permission
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(
              content: Text(
                'Storage access permission is required to open folders',
              ),
              duration: Duration(seconds: 3),
            ),
          );
        }
        return;
      }
    }

    // Try to open the download folder
    final downloadPath = '/storage/emulated/0/Download/ia-get/$identifier';

    try {
      final result = await OpenFile.open(downloadPath);
      if (mounted && result.type != ResultType.done) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Could not open folder: ${result.message}')),
        );
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(
          context,
        ).showSnackBar(SnackBar(content: Text('Error opening folder: $e')));
      }
    }
  }
}
