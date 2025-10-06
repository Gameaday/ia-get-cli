import 'dart:typed_data';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:video_player/video_player.dart';
import 'package:chewie/chewie.dart';
import 'package:path_provider/path_provider.dart';

/// Widget for previewing video files
/// 
/// Displays video content with full playback controls using Chewie player:
/// - Play/pause toggle
/// - Seek bar with duration display
/// - Fullscreen mode
/// - Volume control
/// - Playback speed control
/// - Loading and error states
/// 
/// Uses Chewie wrapper around VideoPlayerController for better UX.
class VideoPreviewWidget extends StatefulWidget {
  /// Raw video bytes to display
  final Uint8List videoBytes;
  
  /// Name of the video file (for display)
  final String fileName;
  
  /// Whether to autoplay video (default: false)
  final bool autoPlay;
  
  /// Whether to loop video (default: false)
  final bool looping;

  const VideoPreviewWidget({
    super.key,
    required this.videoBytes,
    required this.fileName,
    this.autoPlay = false,
    this.looping = false,
  });

  @override
  State<VideoPreviewWidget> createState() => _VideoPreviewWidgetState();
}

class _VideoPreviewWidgetState extends State<VideoPreviewWidget> {
  VideoPlayerController? _videoPlayerController;
  ChewieController? _chewieController;
  String? _errorMessage;
  bool _isLoading = true;
  File? _tempVideoFile;

  @override
  void initState() {
    super.initState();
    _initializePlayer();
  }

  /// Initialize video player from bytes
  /// 
  /// Creates temporary file from bytes since VideoPlayerController
  /// doesn't support direct byte array loading.
  Future<void> _initializePlayer() async {
    try {
      setState(() {
        _isLoading = true;
        _errorMessage = null;
      });

      // Create temporary file from bytes
      final tempDir = await getTemporaryDirectory();
      final tempFile = File('${tempDir.path}/${widget.fileName}');
      await tempFile.writeAsBytes(widget.videoBytes);
      _tempVideoFile = tempFile;

      // Initialize video player
      _videoPlayerController = VideoPlayerController.file(tempFile);
      await _videoPlayerController!.initialize();

      // Create Chewie controller with controls
      _chewieController = ChewieController(
        videoPlayerController: _videoPlayerController!,
        autoPlay: widget.autoPlay,
        looping: widget.looping,
        autoInitialize: true,
        allowFullScreen: true,
        allowMuting: true,
        allowPlaybackSpeedChanging: true,
        showControls: true,
        materialProgressColors: ChewieProgressColors(
          playedColor: Colors.blue,
          handleColor: Colors.blueAccent,
          backgroundColor: Colors.grey,
          bufferedColor: Colors.lightBlue.withValues(alpha: 0.5),
        ),
        placeholder: Container(
          color: Colors.black,
          child: const Center(
            child: CircularProgressIndicator(),
          ),
        ),
        errorBuilder: (context, errorMessage) {
          return Center(
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                const Icon(
                  Icons.error_outline,
                  color: Colors.red,
                  size: 48,
                ),
                const SizedBox(height: 16),
                Text(
                  'Video playback error',
                  style: TextStyle(
                    color: Colors.grey[300],
                    fontSize: 16,
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const SizedBox(height: 8),
                Text(
                  errorMessage,
                  style: TextStyle(
                    color: Colors.grey[400],
                    fontSize: 14,
                  ),
                  textAlign: TextAlign.center,
                ),
              ],
            ),
          );
        },
      );

      setState(() {
        _isLoading = false;
      });
    } catch (e) {
      setState(() {
        _isLoading = false;
        _errorMessage = 'Failed to load video: ${e.toString()}';
      });
    }
  }

  @override
  void dispose() {
    _chewieController?.dispose();
    _videoPlayerController?.dispose();
    
    // Clean up temporary file
    _tempVideoFile?.delete().catchError((_) {
      // Ignore errors during cleanup
      return _tempVideoFile!;
    });
    
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      color: Colors.black,
      child: Center(
        child: _buildContent(),
      ),
    );
  }

  /// Build appropriate content based on state
  Widget _buildContent() {
    if (_isLoading) {
      return _buildLoadingState();
    }

    if (_errorMessage != null) {
      return _buildErrorState();
    }

    if (_chewieController != null) {
      return _buildVideoPlayer();
    }

    return _buildLoadingState();
  }

  /// Build loading state with spinner
  Widget _buildLoadingState() {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        const CircularProgressIndicator(),
        const SizedBox(height: 16),
        Text(
          'Loading video...',
          style: TextStyle(
            color: Colors.grey[300],
            fontSize: 16,
          ),
        ),
        const SizedBox(height: 8),
        Text(
          widget.fileName,
          style: TextStyle(
            color: Colors.grey[400],
            fontSize: 12,
          ),
          textAlign: TextAlign.center,
        ),
      ],
    );
  }

  /// Build error state with message
  Widget _buildErrorState() {
    return Padding(
      padding: const EdgeInsets.all(24.0),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          const Icon(
            Icons.error_outline,
            color: Colors.red,
            size: 64,
          ),
          const SizedBox(height: 24),
          Text(
            'Unable to play video',
            style: TextStyle(
              color: Colors.grey[300],
              fontSize: 18,
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(height: 16),
          Text(
            _errorMessage!,
            style: TextStyle(
              color: Colors.grey[400],
              fontSize: 14,
            ),
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 24),
          ElevatedButton.icon(
            onPressed: _initializePlayer,
            icon: const Icon(Icons.refresh),
            label: const Text('Retry'),
            style: ElevatedButton.styleFrom(
              backgroundColor: Colors.blue,
              foregroundColor: Colors.white,
              padding: const EdgeInsets.symmetric(
                horizontal: 24,
                vertical: 12,
              ),
            ),
          ),
        ],
      ),
    );
  }

  /// Build video player with Chewie controls
  Widget _buildVideoPlayer() {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        // Video player with controls
        Expanded(
          child: Chewie(
            controller: _chewieController!,
          ),
        ),
        
        // Video info footer
        _buildVideoInfo(),
      ],
    );
  }

  /// Build video information footer
  Widget _buildVideoInfo() {
    final duration = _videoPlayerController?.value.duration;
    final size = widget.videoBytes.lengthInBytes;
    
    return Container(
      padding: const EdgeInsets.all(16),
      color: Colors.black87,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            widget.fileName,
            style: const TextStyle(
              color: Colors.white,
              fontSize: 14,
              fontWeight: FontWeight.bold,
            ),
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
          ),
          const SizedBox(height: 8),
          Row(
            children: [
              // Duration
              if (duration != null) ...[
                const Icon(
                  Icons.timer_outlined,
                  size: 16,
                  color: Colors.grey,
                ),
                const SizedBox(width: 4),
                Text(
                  _formatDuration(duration),
                  style: const TextStyle(
                    color: Colors.grey,
                    fontSize: 12,
                  ),
                ),
                const SizedBox(width: 16),
              ],
              
              // File size
              const Icon(
                Icons.storage_outlined,
                size: 16,
                color: Colors.grey,
              ),
              const SizedBox(width: 4),
              Text(
                _formatFileSize(size),
                style: const TextStyle(
                  color: Colors.grey,
                  fontSize: 12,
                ),
              ),
              
              const Spacer(),
              
              // Resolution info
              if (_videoPlayerController?.value.size != null) ...[
                Text(
                  '${_videoPlayerController!.value.size.width.toInt()}x'
                  '${_videoPlayerController!.value.size.height.toInt()}',
                  style: const TextStyle(
                    color: Colors.grey,
                    fontSize: 12,
                  ),
                ),
              ],
            ],
          ),
          
          // Help text
          const SizedBox(height: 12),
          Row(
            children: [
              Icon(
                Icons.info_outline,
                size: 14,
                color: Colors.grey[600],
              ),
              const SizedBox(width: 4),
              Expanded(
                child: Text(
                  'Tap for controls • Double-tap for fullscreen',
                  style: TextStyle(
                    color: Colors.grey[600],
                    fontSize: 11,
                    fontStyle: FontStyle.italic,
                  ),
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }

  /// Format duration to MM:SS
  String _formatDuration(Duration duration) {
    final minutes = duration.inMinutes;
    final seconds = duration.inSeconds % 60;
    return '${minutes.toString().padLeft(2, '0')}:${seconds.toString().padLeft(2, '0')}';
  }

  /// Format file size to human-readable format
  String _formatFileSize(int bytes) {
    if (bytes < 1024) {
      return '$bytes B';
    } else if (bytes < 1024 * 1024) {
      return '${(bytes / 1024).toStringAsFixed(1)} KB';
    } else if (bytes < 1024 * 1024 * 1024) {
      return '${(bytes / (1024 * 1024)).toStringAsFixed(1)} MB';
    } else {
      return '${(bytes / (1024 * 1024 * 1024)).toStringAsFixed(1)} GB';
    }
  }
}
