import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../services/ia_get_service.dart';
import '../widgets/archive_info_widget.dart';
import '../widgets/file_list_widget.dart';
import '../widgets/download_controls_widget.dart';
import '../widgets/download_manager_widget.dart';

/// Screen showing archive details with files and download options
class ArchiveDetailScreen extends StatelessWidget {
  const ArchiveDetailScreen({super.key});

  /// Route name for navigation tracking and state restoration
  static const routeName = '/archive-detail';

  @override
  Widget build(BuildContext context) {
    return PopScope(
      canPop: true,
      onPopInvokedWithResult: (didPop, result) {
        if (didPop) {
          // Clear metadata when going back to search
          // Use Provider.of with listen: false for safer context access in callbacks
          final service = Provider.of<IaGetService>(context, listen: false);
          service.clearMetadata();
        }
      },
      child: Scaffold(
        appBar: AppBar(
          title: Consumer<IaGetService>(
            builder: (context, service, child) {
              return Text(
                service.currentMetadata?.identifier ?? 'Archive Details',
                overflow: TextOverflow.ellipsis,
              );
            },
          ),
        ),
        body: Consumer<IaGetService>(
          builder: (context, service, child) {
            if (service.currentMetadata == null) {
              // If no metadata, go back to search
              WidgetsBinding.instance.addPostFrameCallback((_) {
                Navigator.of(context).pop();
              });
              return const Center(child: CircularProgressIndicator());
            }

            return Column(
              children: [
                // Archive information
                ArchiveInfoWidget(metadata: service.currentMetadata!),

                // File list (with integrated filter controls)
                Expanded(child: FileListWidget(files: service.filteredFiles)),

                // Download controls
                const DownloadControlsWidget(),

                // Active downloads manager
                const DownloadManagerWidget(),
              ],
            );
          },
        ),
      ),
    );
  }
}
