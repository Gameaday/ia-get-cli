import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../services/archive_service.dart';
import '../services/history_service.dart';
import '../utils/semantic_colors.dart';
import '../widgets/search_bar_widget.dart';
import '../widgets/search_suggestion_card.dart';
import '../widgets/download_manager_widget.dart';
import 'archive_detail_screen.dart';
import 'download_screen.dart';
import 'help_screen.dart';
import 'history_screen.dart';
import 'settings_screen.dart';

class HomeScreen extends StatefulWidget {
  const HomeScreen({super.key});

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  bool _hasNavigated = false;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted) return;
      
      // Initialize the services
      context.read<ArchiveService>().initialize();
      context.read<HistoryService>().loadHistory();

      // Listen for metadata changes to navigate to detail screen
      context.read<ArchiveService>().addListener(_onServiceChanged);
    });
  }

  @override
  void dispose() {
    // Safe removal - only if context is still valid
    try {
      context.read<ArchiveService>().removeListener(_onServiceChanged);
    } catch (e) {
      // Context may already be invalid during disposal
      debugPrint('Warning: Could not remove listener during dispose: $e');
    }
    super.dispose();
  }

  void _onServiceChanged() {
    final service = context.read<ArchiveService>();

    // Navigate to detail screen only when metadata is successfully loaded
    // Check that we have metadata AND no error AND not currently loading
    if (service.currentMetadata != null && 
        service.error == null && 
        !service.isLoading &&
        mounted && 
        !_hasNavigated) {
      _hasNavigated = true;

      Navigator.of(context)
          .push(
            MaterialPageRoute(
              builder: (context) => const ArchiveDetailScreen(),
              settings: const RouteSettings(name: ArchiveDetailScreen.routeName),
            ),
          )
          .then((_) {
            // Reset flag when returning from detail screen
            _hasNavigated = false;
          });
    } else if (service.currentMetadata == null) {
      // Reset flag when metadata is cleared
      _hasNavigated = false;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Search'),
        actions: [
          IconButton(
            icon: const Icon(Icons.history),
            onPressed: () {
              Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (_) => const HistoryScreen(),
                  settings: const RouteSettings(name: HistoryScreen.routeName),
                ),
              );
            },
          ),
          IconButton(
            icon: const Icon(Icons.settings),
            onPressed: () {
              Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (_) => const SettingsScreen(),
                  settings: const RouteSettings(name: '/settings'),
                ),
              );
            },
          ),
          IconButton(
            icon: const Icon(Icons.help_outline),
            onPressed: () {
              Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (_) => const HelpScreen(),
                  settings: const RouteSettings(name: '/help'),
                ),
              );
            },
          ),
          IconButton(
            icon: const Icon(Icons.download_rounded),
            onPressed: () {
              Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (_) => const DownloadScreen(useBackground: true),
                  settings: const RouteSettings(name: DownloadScreen.routeName),
                ),
              );
            },
          ),
        ],
      ),
      body: Consumer<ArchiveService>(
        builder: (context, service, child) {
          if (!service.isInitialized) {
            // Show error if initialization failed
            if (service.error != null) {
              return Center(
                child: Padding(
                  padding: const EdgeInsets.all(24.0),
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Icon(
                        Icons.error_outline,
                        color: SemanticColors.error(context),
                        size: 64,
                      ),
                      const SizedBox(height: 24),
                      const Text(
                        'Initialization Failed',
                        style: TextStyle(
                          fontSize: 20,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      const SizedBox(height: 16),
                      Text(
                        service.error!,
                        textAlign: TextAlign.center,
                        style: const TextStyle(fontSize: 14),
                      ),
                      const SizedBox(height: 24),
                      ElevatedButton(
                        onPressed: () {
                          // Try to re-initialize
                          service.initialize();
                        },
                        child: const Text('Retry'),
                      ),
                    ],
                  ),
                ),
              );
            }
            
            // Show loading if still initializing
            return const Center(
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  CircularProgressIndicator(),
                  SizedBox(height: 16),
                  Text('Initializing Internet Archive Helper...'),
                ],
              ),
            );
          }

          // Safety check: if we're on home screen, metadata should be cleared
          // This prevents black screen if we somehow navigate back without clearing metadata
          WidgetsBinding.instance.addPostFrameCallback((_) {
            if (mounted && service.currentMetadata != null && !_hasNavigated) {
              // We have metadata but haven't navigated - this shouldn't happen normally
              // Clear it to ensure consistent state
              service.clearMetadata();
            }
          });

          return Column(
            children: [
              // Search bar
              const SearchBarWidget(),

              // Error display
              if (service.error != null)
                Container(
                  width: double.infinity,
                  padding: const EdgeInsets.all(16),
                  margin: const EdgeInsets.all(8),
                  decoration: BoxDecoration(
                    color: Theme.of(context).colorScheme.errorContainer,
                    borderRadius: BorderRadius.circular(8),
                    border: Border.all(color: Theme.of(context).colorScheme.error),
                  ),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Row(
                        children: [
                          Icon(Icons.error_outline, color: Theme.of(context).colorScheme.onErrorContainer),
                          const SizedBox(width: 8),
                          Expanded(
                            child: Text(
                              service.error!,
                              style: TextStyle(color: Theme.of(context).colorScheme.onErrorContainer),
                            ),
                          ),
                        ],
                      ),
                    ],
                  ),
                ),

              // Search suggestions
              if (service.suggestions.isNotEmpty)
                Expanded(
                  child: ListView(
                    padding: const EdgeInsets.only(top: 8, bottom: 8),
                    children: [
                      Padding(
                        padding: const EdgeInsets.fromLTRB(16, 8, 16, 8),
                        child: Text(
                          'Suggestions:',
                          style: TextStyle(
                            fontSize: 18,
                            fontWeight: FontWeight.bold,
                            color: Theme.of(context).colorScheme.onSurface,
                          ),
                        ),
                      ),
                      ...service.suggestions.map((suggestion) {
                        return SearchSuggestionCard(
                          suggestion: suggestion,
                          onTap: () {
                            // Clear error and suggestions before fetching
                            service.clearMetadata();
                            // Fetch metadata for the suggested archive
                            service.fetchMetadata(suggestion.identifier);
                          },
                        );
                      }),
                    ],
                  ),
                ),

              // Loading indicator
              if (service.isLoading) const LinearProgressIndicator(),

              // Empty state when not loading and no metadata
              if (!service.isLoading &&
                  service.currentMetadata == null &&
                  service.suggestions.isEmpty &&
                  service.error == null)
                Expanded(
                  child: Center(
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Icon(
                          Icons.search,
                          size: 64,
                          color: SemanticColors.disabled(context),
                        ),
                        const SizedBox(height: 16),
                        Text(
                          'Search for an Internet Archive identifier',
                          style: TextStyle(
                            fontSize: 16,
                            color: SemanticColors.subtitle(context),
                          ),
                        ),
                        const SizedBox(height: 8),
                        Text(
                          'e.g., "commute_test" or "nasa_images"',
                          style: TextStyle(
                            fontSize: 14,
                            color: SemanticColors.hint(context),
                          ),
                        ),
                      ],
                    ),
                  ),
                ),

              // Active downloads manager at bottom
              const DownloadManagerWidget(),
            ],
          );
        },
      ),
    );
  }
}
