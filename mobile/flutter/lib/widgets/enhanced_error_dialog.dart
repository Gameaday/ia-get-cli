import 'package:flutter/material.dart';
import '../models/download_error.dart';

/// Enhanced error dialog with categorization and retry options
/// 
/// Features:
/// - Color-coded by error category
/// - Icon visual representation
/// - Detailed error message
/// - Suggested actions
/// - Technical details (expandable)
/// - Retry button (if retryable)
/// - Dismiss button
class EnhancedErrorDialog extends StatefulWidget {
  final DownloadError error;
  final VoidCallback? onRetry;
  final VoidCallback? onDismiss;

  const EnhancedErrorDialog({
    super.key,
    required this.error,
    this.onRetry,
    this.onDismiss,
  });

  /// Show the error dialog
  static Future<void> show(
    BuildContext context, {
    required DownloadError error,
    VoidCallback? onRetry,
  }) {
    return showDialog<void>(
      context: context,
      builder: (context) => EnhancedErrorDialog(
        error: error,
        onRetry: onRetry,
        onDismiss: () => Navigator.of(context).pop(),
      ),
    );
  }

  @override
  State<EnhancedErrorDialog> createState() => _EnhancedErrorDialogState();
}

class _EnhancedErrorDialogState extends State<EnhancedErrorDialog> {
  bool _showTechnicalDetails = false;

  @override
  Widget build(BuildContext context) {
    final category = widget.error.category;
    final color = _getCategoryColor(category);

    return AlertDialog(
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(16),
        side: BorderSide(color: color.withAlpha(76), width: 2),
      ),
      titlePadding: EdgeInsets.zero,
      title: _buildTitle(color),
      content: _buildContent(),
      actions: _buildActions(context),
    );
  }

  Widget _buildTitle(Color color) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: color.withAlpha(25),
        borderRadius: const BorderRadius.only(
          topLeft: Radius.circular(14),
          topRight: Radius.circular(14),
        ),
      ),
      child: Row(
        children: [
          Text(
            widget.error.category.icon,
            style: const TextStyle(fontSize: 24),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Text(
              widget.error.category.displayName,
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
                color: color,
              ),
            ),
          ),
          if (widget.error.retryCount > 0)
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
              decoration: BoxDecoration(
                color: Colors.orange.withAlpha(25),
                borderRadius: BorderRadius.circular(12),
                border: Border.all(color: Colors.orange.withAlpha(76)),
              ),
              child: Text(
                'Retry ${widget.error.retryCount}',
                style: const TextStyle(
                  fontSize: 11,
                  fontWeight: FontWeight.w600,
                  color: Colors.orange,
                ),
              ),
            ),
        ],
      ),
    );
  }

  Widget _buildContent() {
    return SingleChildScrollView(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Error message
          Text(
            widget.error.message,
            style: const TextStyle(
              fontSize: 15,
              fontWeight: FontWeight.w600,
            ),
          ),
          const SizedBox(height: 12),

          // Suggested action
          Container(
            padding: const EdgeInsets.all(12),
            decoration: BoxDecoration(
              color: Colors.blue.withAlpha(25),
              borderRadius: BorderRadius.circular(8),
              border: Border.all(color: Colors.blue.withAlpha(76)),
            ),
            child: Row(
              children: [
                const Icon(Icons.lightbulb_outline, size: 18, color: Colors.blue),
                const SizedBox(width: 8),
                Expanded(
                  child: Text(
                    widget.error.suggestedAction,
                    style: const TextStyle(fontSize: 13),
                  ),
                ),
              ],
            ),
          ),

          // Technical details (expandable)
          if (widget.error.technicalDetails != null) ...[
            const SizedBox(height: 12),
            InkWell(
              onTap: () {
                setState(() {
                  _showTechnicalDetails = !_showTechnicalDetails;
                });
              },
              child: Row(
                children: [
                  Icon(
                    _showTechnicalDetails
                        ? Icons.expand_less
                        : Icons.expand_more,
                    size: 20,
                  ),
                  const SizedBox(width: 4),
                  Text(
                    _showTechnicalDetails ? 'Hide Details' : 'Show Details',
                    style: TextStyle(
                      fontSize: 13,
                      color: Colors.grey.shade700,
                      fontWeight: FontWeight.w500,
                    ),
                  ),
                ],
              ),
            ),
            if (_showTechnicalDetails) ...[
              const SizedBox(height: 8),
              Container(
                padding: const EdgeInsets.all(10),
                decoration: BoxDecoration(
                  color: Colors.grey.shade100,
                  borderRadius: BorderRadius.circular(6),
                  border: Border.all(color: Colors.grey.shade300),
                ),
                child: Text(
                  widget.error.technicalDetails!,
                  style: TextStyle(
                    fontSize: 11,
                    fontFamily: 'monospace',
                    color: Colors.grey.shade800,
                  ),
                ),
              ),
            ],
          ],

          // Status code if available
          if (widget.error.statusCode != null) ...[
            const SizedBox(height: 8),
            Text(
              'Status Code: ${widget.error.statusCode}',
              style: TextStyle(
                fontSize: 11,
                color: Colors.grey.shade600,
              ),
            ),
          ],
        ],
      ),
    );
  }

  List<Widget> _buildActions(BuildContext context) {
    return [
      // Dismiss button
      TextButton(
        onPressed: widget.onDismiss ?? () => Navigator.of(context).pop(),
        child: const Text('Dismiss'),
      ),

      // Retry button (if retryable)
      if (widget.error.category.isRetryable && widget.onRetry != null)
        FilledButton.icon(
          onPressed: () {
            Navigator.of(context).pop();
            widget.onRetry?.call();
          },
          icon: const Icon(Icons.refresh, size: 18),
          label: const Text('Retry'),
        ),
    ];
  }

  Color _getCategoryColor(DownloadErrorCategory category) {
    switch (category) {
      case DownloadErrorCategory.network:
        return Colors.orange;
      case DownloadErrorCategory.server:
        return Colors.red;
      case DownloadErrorCategory.rateLimited:
        return Colors.purple;
      case DownloadErrorCategory.storage:
        return Colors.amber;
      case DownloadErrorCategory.permission:
        return Colors.red.shade900;
      case DownloadErrorCategory.corruption:
        return Colors.red.shade700;
      case DownloadErrorCategory.cancelled:
        return Colors.grey;
      case DownloadErrorCategory.unknown:
        return Colors.blueGrey;
    }
  }
}

/// Compact error badge for inline display
class ErrorBadge extends StatelessWidget {
  final DownloadError error;
  final VoidCallback? onTap;

  const ErrorBadge({
    super.key,
    required this.error,
    this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    final color = _getCategoryColor(error.category);

    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(10),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
        decoration: BoxDecoration(
          color: color.withAlpha(25),
          borderRadius: BorderRadius.circular(10),
          border: Border.all(color: color.withAlpha(76), width: 1),
        ),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(
              error.category.icon,
              style: const TextStyle(fontSize: 12),
            ),
            const SizedBox(width: 4),
            Text(
              error.category.displayName,
              style: TextStyle(
                fontSize: 10,
                fontWeight: FontWeight.w600,
                color: color,
              ),
            ),
            if (error.retryCount > 0) ...[
              const SizedBox(width: 4),
              Text(
                '(${error.retryCount})',
                style: TextStyle(
                  fontSize: 9,
                  color: color.withAlpha(178),
                ),
              ),
            ],
          ],
        ),
      ),
    );
  }

  Color _getCategoryColor(DownloadErrorCategory category) {
    switch (category) {
      case DownloadErrorCategory.network:
        return Colors.orange;
      case DownloadErrorCategory.server:
        return Colors.red;
      case DownloadErrorCategory.rateLimited:
        return Colors.purple;
      case DownloadErrorCategory.storage:
        return Colors.amber;
      case DownloadErrorCategory.permission:
        return Colors.red.shade900;
      case DownloadErrorCategory.corruption:
        return Colors.red.shade700;
      case DownloadErrorCategory.cancelled:
        return Colors.grey;
      case DownloadErrorCategory.unknown:
        return Colors.blueGrey;
    }
  }
}
