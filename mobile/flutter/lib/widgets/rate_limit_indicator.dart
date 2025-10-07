import 'package:flutter/material.dart';
import '../models/rate_limit_status.dart';

/// Compact rate limit status indicator for mobile
/// 
/// Shows current rate limiting status with color-coded indicator
/// Expandable to show details
class RateLimitIndicator extends StatelessWidget {
  final RateLimitStatus status;
  final bool showDetails;

  const RateLimitIndicator({
    super.key,
    required this.status,
    this.showDetails = false,
  });

  @override
  Widget build(BuildContext context) {
    // Don't show anything if rate limiting is not active and no queue
    if (!status.isRateLimiting && !status.hasRetryAfter) {
      return const SizedBox.shrink();
    }

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
      decoration: BoxDecoration(
        color: Color(status.level.colorValue).withAlpha(25),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(
          color: Color(status.level.colorValue).withAlpha(76),
          width: 1,
        ),
      ),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          _buildCompactStatus(context),
          if (showDetails) ...[
            const SizedBox(height: 8),
            _buildDetails(context),
          ],
        ],
      ),
    );
  }

  /// Build compact status row
  Widget _buildCompactStatus(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        Text(
          status.level.icon,
          style: const TextStyle(fontSize: 16),
        ),
        const SizedBox(width: 8),
        Flexible(
          child: Text(
            status.message,
            style: TextStyle(
              fontSize: 13,
              fontWeight: FontWeight.w600,
              color: Color(status.level.colorValue),
            ),
            overflow: TextOverflow.ellipsis,
          ),
        ),
        if (status.hasRetryAfter) ...[
          const SizedBox(width: 8),
          _buildCountdown(),
        ],
      ],
    );
  }

  /// Build countdown timer for retry-after
  Widget _buildCountdown() {
    final remaining = status.retryAfterRemaining ?? 0;
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
      decoration: BoxDecoration(
        color: Color(status.level.colorValue).withAlpha(51),
        borderRadius: BorderRadius.circular(4),
      ),
      child: Text(
        '${remaining}s',
        style: TextStyle(
          fontSize: 11,
          fontWeight: FontWeight.bold,
          color: Color(status.level.colorValue),
        ),
      ),
    );
  }

  /// Build detailed status information
  Widget _buildDetails(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        _buildDetailRow(
          icon: Icons.play_arrow,
          label: 'Active',
          value: '${status.activeRequests}/${status.maxConcurrent}',
        ),
        const SizedBox(height: 4),
        _buildDetailRow(
          icon: Icons.queue,
          label: 'Queued',
          value: '${status.queuedRequests}',
        ),
        const SizedBox(height: 4),
        _buildDetailRow(
          icon: Icons.speed,
          label: 'Utilization',
          value: '${status.utilizationPercentage.toStringAsFixed(0)}%',
        ),
        if (status.hasRetryAfter) ...[
          const SizedBox(height: 4),
          _buildDetailRow(
            icon: Icons.timer,
            label: 'Server Delay',
            value: '${status.retryAfterSeconds}s',
          ),
        ],
      ],
    );
  }

  /// Build a detail row
  Widget _buildDetailRow({
    required IconData icon,
    required String label,
    required String value,
  }) {
    return Row(
      children: [
        Icon(icon, size: 14, color: Colors.grey.shade600),
        const SizedBox(width: 6),
        Expanded(
          child: Text(
            label,
            style: TextStyle(
              fontSize: 11,
              color: Colors.grey.shade700,
            ),
          ),
        ),
        Text(
          value,
          style: const TextStyle(
            fontSize: 11,
            fontWeight: FontWeight.w600,
          ),
        ),
      ],
    );
  }
}

/// Mini rate limit badge (for compact display)
class RateLimitBadge extends StatelessWidget {
  final RateLimitStatus status;

  const RateLimitBadge({
    super.key,
    required this.status,
  });

  @override
  Widget build(BuildContext context) {
    // Don't show if not rate limiting
    if (!status.isRateLimiting && !status.hasRetryAfter) {
      return const SizedBox.shrink();
    }

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 3),
      decoration: BoxDecoration(
        color: Color(status.level.colorValue).withAlpha(25),
        borderRadius: BorderRadius.circular(10),
        border: Border.all(
          color: Color(status.level.colorValue).withAlpha(76),
          width: 1,
        ),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(
            status.level.icon,
            style: const TextStyle(fontSize: 10),
          ),
          if (status.queuedRequests > 0) ...[
            const SizedBox(width: 3),
            Text(
              '${status.queuedRequests}',
              style: TextStyle(
                fontSize: 9,
                fontWeight: FontWeight.bold,
                color: Color(status.level.colorValue),
              ),
            ),
          ],
        ],
      ),
    );
  }
}
