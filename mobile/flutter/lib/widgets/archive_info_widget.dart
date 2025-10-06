import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../models/archive_metadata.dart';
import '../services/archive_service.dart';
import '../models/cached_metadata.dart';

class ArchiveInfoWidget extends StatelessWidget {
  final ArchiveMetadata metadata;

  const ArchiveInfoWidget({super.key, required this.metadata});

  @override
  Widget build(BuildContext context) {
    final archiveService = Provider.of<ArchiveService>(context, listen: false);

    return Card(
      margin: const EdgeInsets.all(16),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                const Icon(Icons.archive, color: Colors.blue),
                const SizedBox(width: 8),
                Expanded(
                  child: Text(
                    metadata.title ?? metadata.identifier,
                    style: const TextStyle(
                      fontSize: 18,
                      fontWeight: FontWeight.bold,
                    ),
                    maxLines: 2,
                    overflow: TextOverflow.ellipsis,
                  ),
                ),
                // Offline indicator and pin button
                FutureBuilder<bool>(
                  future: archiveService.isCached(metadata.identifier),
                  builder: (context, snapshot) {
                    final isCached = snapshot.data ?? false;
                    if (!isCached) return const SizedBox.shrink();

                    return Row(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        // Offline badge
                        Tooltip(
                          message: 'Available offline',
                          child: Container(
                            padding: const EdgeInsets.symmetric(
                              horizontal: 8,
                              vertical: 4,
                            ),
                            decoration: BoxDecoration(
                              color: Colors.green.shade100,
                              borderRadius: BorderRadius.circular(12),
                            ),
                            child: Row(
                              mainAxisSize: MainAxisSize.min,
                              children: [
                                Icon(
                                  Icons.offline_pin,
                                  size: 14,
                                  color: Colors.green.shade700,
                                ),
                                const SizedBox(width: 4),
                                Text(
                                  'Offline',
                                  style: TextStyle(
                                    fontSize: 11,
                                    fontWeight: FontWeight.bold,
                                    color: Colors.green.shade700,
                                  ),
                                ),
                              ],
                            ),
                          ),
                        ),
                        const SizedBox(width: 8),
                        // Pin/Unpin button
                        FutureBuilder<CachedMetadata?>(
                          future: archiveService
                              .getCachedMetadata(metadata.identifier)
                              .then(
                                (m) => m != null
                                    ? CachedMetadata.fromMetadata(m)
                                    : null,
                              ),
                          builder: (context, cacheSnapshot) {
                            final cachedMeta = cacheSnapshot.data;
                            final isPinned = cachedMeta?.isPinned ?? false;

                            return IconButton(
                              icon: Icon(
                                isPinned
                                    ? Icons.push_pin
                                    : Icons.push_pin_outlined,
                                color: isPinned ? Colors.orange : Colors.grey,
                              ),
                              tooltip: isPinned
                                  ? 'Unpin archive'
                                  : 'Pin archive',
                              onPressed: () async {
                                await archiveService.togglePin(
                                  metadata.identifier,
                                );
                                // Rebuild UI
                                (context as Element).markNeedsBuild();
                              },
                            );
                          },
                        ),
                        // Sync button
                        IconButton(
                          icon: const Icon(Icons.sync, color: Colors.blue),
                          tooltip: 'Sync metadata',
                          onPressed: () async {
                            ScaffoldMessenger.of(context).showSnackBar(
                              const SnackBar(
                                content: Text('Syncing metadata...'),
                                duration: Duration(seconds: 1),
                              ),
                            );
                            try {
                              await archiveService.syncMetadata(
                                metadata.identifier,
                              );
                              if (context.mounted) {
                                ScaffoldMessenger.of(context).showSnackBar(
                                  const SnackBar(
                                    content: Text(
                                      'Metadata synced successfully',
                                    ),
                                    duration: Duration(seconds: 2),
                                  ),
                                );
                              }
                            } catch (e) {
                              if (context.mounted) {
                                ScaffoldMessenger.of(context).showSnackBar(
                                  SnackBar(
                                    content: Text('Sync failed: $e'),
                                    backgroundColor: Colors.red,
                                  ),
                                );
                              }
                            }
                          },
                        ),
                      ],
                    );
                  },
                ),
              ],
            ),
            if (metadata.description != null) ...[
              const SizedBox(height: 8),
              Text(
                metadata.description!,
                style: TextStyle(color: Colors.grey.shade600),
                maxLines: 3,
                overflow: TextOverflow.ellipsis,
              ),
            ],
            const SizedBox(height: 12),
            Row(
              children: [
                if (metadata.creator != null) ...[
                  Icon(Icons.person, size: 16, color: Colors.grey.shade600),
                  const SizedBox(width: 4),
                  Expanded(
                    child: Text(
                      metadata.creator!,
                      style: TextStyle(
                        fontSize: 12,
                        color: Colors.grey.shade600,
                      ),
                      overflow: TextOverflow.ellipsis,
                    ),
                  ),
                ],
                if (metadata.date != null) ...[
                  const SizedBox(width: 16),
                  Icon(
                    Icons.calendar_today,
                    size: 16,
                    color: Colors.grey.shade600,
                  ),
                  const SizedBox(width: 4),
                  Text(
                    metadata.date!,
                    style: TextStyle(fontSize: 12, color: Colors.grey.shade600),
                  ),
                ],
              ],
            ),
            const SizedBox(height: 8),
            Row(
              children: [
                Icon(Icons.folder, size: 16, color: Colors.grey.shade600),
                const SizedBox(width: 4),
                Text(
                  '${metadata.totalFiles} files',
                  style: TextStyle(fontSize: 12, color: Colors.grey.shade600),
                ),
                const SizedBox(width: 16),
                Icon(Icons.storage, size: 16, color: Colors.grey.shade600),
                const SizedBox(width: 4),
                Text(
                  _formatSize(metadata.totalSize),
                  style: TextStyle(fontSize: 12, color: Colors.grey.shade600),
                ),
              ],
            ),
            // Cache sync status
            FutureBuilder<CachedMetadata?>(
              future: archiveService
                  .getCachedMetadata(metadata.identifier)
                  .then(
                    (m) => m != null ? CachedMetadata.fromMetadata(m) : null,
                  ),
              builder: (context, snapshot) {
                final cachedMeta = snapshot.data;
                if (cachedMeta == null) return const SizedBox.shrink();

                return Padding(
                  padding: const EdgeInsets.only(top: 8),
                  child: Row(
                    children: [
                      Icon(Icons.sync, size: 14, color: Colors.blue.shade400),
                      const SizedBox(width: 4),
                      Text(
                        cachedMeta.syncStatusString,
                        style: TextStyle(
                          fontSize: 11,
                          color: Colors.blue.shade600,
                        ),
                      ),
                      if (cachedMeta.isPinned) ...[
                        const SizedBox(width: 12),
                        Icon(
                          Icons.push_pin,
                          size: 14,
                          color: Colors.orange.shade400,
                        ),
                        const SizedBox(width: 4),
                        Text(
                          'Pinned',
                          style: TextStyle(
                            fontSize: 11,
                            color: Colors.orange.shade600,
                          ),
                        ),
                      ],
                    ],
                  ),
                );
              },
            ),
          ],
        ),
      ),
    );
  }

  String _formatSize(int bytes) {
    const units = ['B', 'KB', 'MB', 'GB', 'TB'];
    double size = bytes.toDouble();
    int unitIndex = 0;

    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024;
      unitIndex++;
    }

    return '${size.toStringAsFixed(size >= 100 ? 0 : 1)} ${units[unitIndex]}';
  }
}
