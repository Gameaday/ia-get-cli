import 'package:flutter/material.dart';
import '../models/download_progress_info.dart';
import '../utils/file_utils.dart';

/// Enhanced progress display for a single download
/// 
/// Shows speed, ETA, and detailed file progress in a compact mobile layout.
/// Expandable to show more details.
class EnhancedProgressCard extends StatelessWidget {
  final DownloadProgressInfo progressInfo;
  final bool isExpanded;
  final VoidCallback? onToggleExpanded;

  const EnhancedProgressCard({
    super.key,
    required this.progressInfo,
    this.isExpanded = false,
    this.onToggleExpanded,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        _buildCompactInfo(context),
        if (isExpanded) ...[
          const SizedBox(height: 8),
          _buildDetailedInfo(context),
        ],
      ],
    );
  }

  /// Build compact info row (always visible)
  Widget _buildCompactInfo(BuildContext context) {
    return Row(
      children: [
        // Speed indicator - always show, even if 0
        Icon(
          progressInfo.isThrottled ? Icons.speed : Icons.bolt,
          size: 16,
          color: progressInfo.isThrottled 
              ? Colors.orange 
              : (progressInfo.hasSpeedData ? Colors.blue : Colors.grey),
        ),
        const SizedBox(width: 4),
        Text(
          progressInfo.hasSpeedData 
              ? progressInfo.formattedCurrentSpeed 
              : 'Starting...',
          style: TextStyle(
            fontSize: 13,
            fontWeight: FontWeight.w600,
            color: progressInfo.hasSpeedData ? null : Colors.grey.shade600,
          ),
        ),
        const SizedBox(width: 12),
        
        // ETA - show calculating if not available
        const Icon(Icons.timer_outlined, size: 16, color: Colors.grey),
        const SizedBox(width: 4),
        Text(
          progressInfo.formattedEta,
          style: TextStyle(
            fontSize: 13,
            color: Colors.grey.shade700,
          ),
        ),
        const SizedBox(width: 12),
        
        // File count
        const Icon(Icons.insert_drive_file_outlined, size: 16, color: Colors.grey),
        const SizedBox(width: 4),
        Text(
          progressInfo.formattedFileProgress,
          style: TextStyle(
            fontSize: 13,
            color: Colors.grey.shade700,
          ),
        ),
        
        const Spacer(),
        
        // Expand/collapse button
        if (onToggleExpanded != null)
          IconButton(
            icon: Icon(
              isExpanded ? Icons.expand_less : Icons.expand_more,
              size: 20,
            ),
            padding: EdgeInsets.zero,
            constraints: const BoxConstraints(),
            onPressed: onToggleExpanded,
          ),
      ],
    );
  }

  /// Build detailed info section (shown when expanded)
  Widget _buildDetailedInfo(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: Colors.grey.shade100,
        borderRadius: BorderRadius.circular(8),
      ),
      child: Column(
        children: [
          _buildDetailRow(
            icon: Icons.speed,
            label: 'Current Speed',
            value: progressInfo.hasSpeedData 
                ? progressInfo.formattedCurrentSpeed 
                : 'Starting...',
          ),
          const SizedBox(height: 8),
          _buildDetailRow(
            icon: Icons.show_chart,
            label: 'Average Speed',
            value: progressInfo.hasSpeedData 
                ? progressInfo.formattedAverageSpeed 
                : 'Calculating...',
          ),
          const SizedBox(height: 8),
          _buildDetailRow(
            icon: Icons.access_time,
            label: 'Elapsed',
            value: progressInfo.formattedElapsed,
          ),
          const SizedBox(height: 8),
          _buildDetailRow(
            icon: Icons.data_usage,
            label: 'Downloaded',
            value: '${FileUtils.formatSize(progressInfo.bytesDownloaded)} / ${FileUtils.formatSize(progressInfo.totalBytes)}',
          ),
          if (progressInfo.isThrottled) ...[
            const SizedBox(height: 8),
            Row(
              children: [
                Icon(Icons.warning_amber_rounded, size: 16, color: Colors.orange.shade700),
                const SizedBox(width: 4),
                Text(
                  'Bandwidth throttling active',
                  style: TextStyle(
                    fontSize: 12,
                    color: Colors.orange.shade700,
                    fontStyle: FontStyle.italic,
                  ),
                ),
              ],
            ),
          ],
          if (!progressInfo.hasSpeedData) ...[
            const SizedBox(height: 8),
            Row(
              children: [
                Icon(Icons.info_outline, size: 16, color: Colors.blue.shade700),
                const SizedBox(width: 4),
                Expanded(
                  child: Text(
                    'Speed will be calculated once download starts',
                    style: TextStyle(
                      fontSize: 11,
                      color: Colors.blue.shade700,
                      fontStyle: FontStyle.italic,
                    ),
                  ),
                ),
              ],
            ),
          ],
        ],
      ),
    );
  }

  /// Build a detail row with icon, label, and value
  Widget _buildDetailRow({
    required IconData icon,
    required String label,
    required String value,
  }) {
    return Row(
      children: [
        Icon(icon, size: 16, color: Colors.grey.shade600),
        const SizedBox(width: 8),
        Expanded(
          child: Text(
            label,
            style: TextStyle(
              fontSize: 12,
              color: Colors.grey.shade700,
            ),
          ),
        ),
        Text(
          value,
          style: const TextStyle(
            fontSize: 12,
            fontWeight: FontWeight.w600,
          ),
        ),
      ],
    );
  }
}
