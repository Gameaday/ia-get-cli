import 'package:sqflite/sqflite.dart';
import 'package:path/path.dart';
import 'package:flutter/foundation.dart';

/// Database helper for managing SQLite database operations
/// Used for metadata caching and file preview caching with versioning and migrations
class DatabaseHelper {
  static const String _databaseName = 'ia_get.db';
  static const int _databaseVersion = 3;

  // Table names
  static const String tableCachedMetadata = 'cached_metadata';
  static const String tablePreviewCache = 'preview_cache';
  
  // Singleton pattern
  DatabaseHelper._privateConstructor();
  static final DatabaseHelper instance = DatabaseHelper._privateConstructor();
  
  static Database? _database;

  /// Get database instance, creating it if necessary
  Future<Database> get database async {
    if (_database != null) return _database!;
    _database = await _initDatabase();
    return _database!;
  }

  /// Initialize database with schema
  Future<Database> _initDatabase() async {
    try {
      final databasesPath = await getDatabasesPath();
      final path = join(databasesPath, _databaseName);

      debugPrint('Opening database at: $path');

      return await openDatabase(
        path,
        version: _databaseVersion,
        onCreate: _onCreate,
        onUpgrade: _onUpgrade,
        onConfigure: _onConfigure,
      );
    } catch (e) {
      debugPrint('Error initializing database: $e');
      rethrow;
    }
  }

  /// Configure database settings before opening
  Future<void> _onConfigure(Database db) async {
    // Enable foreign keys
    await db.execute('PRAGMA foreign_keys = ON');
  }

  /// Create database schema
  Future<void> _onCreate(Database db, int version) async {
    debugPrint('Creating database schema version $version');

    // Cached archive metadata table
    await db.execute('''
      CREATE TABLE $tableCachedMetadata (
        identifier TEXT PRIMARY KEY,
        metadata_json TEXT NOT NULL,
        cached_at INTEGER NOT NULL,
        last_accessed INTEGER NOT NULL,
        last_synced INTEGER,
        version INTEGER NOT NULL DEFAULT 1,
        is_pinned INTEGER NOT NULL DEFAULT 0,
        file_count INTEGER NOT NULL DEFAULT 0,
        total_size INTEGER NOT NULL DEFAULT 0,
        creator TEXT,
        title TEXT,
        media_type TEXT,
        etag TEXT,
        UNIQUE(identifier)
      )
    ''');

    // Create indexes for efficient queries
    await db.execute('''
      CREATE INDEX idx_cached_metadata_last_accessed 
      ON $tableCachedMetadata(last_accessed DESC)
    ''');

    await db.execute('''
      CREATE INDEX idx_cached_metadata_is_pinned 
      ON $tableCachedMetadata(is_pinned)
    ''');

    await db.execute('''
      CREATE INDEX idx_cached_metadata_cached_at 
      ON $tableCachedMetadata(cached_at DESC)
    ''');

    // File preview cache table
    await db.execute('''
      CREATE TABLE $tablePreviewCache (
        identifier TEXT NOT NULL,
        file_name TEXT NOT NULL,
        preview_type TEXT NOT NULL,
        text_content TEXT,
        preview_data BLOB,
        cached_at INTEGER NOT NULL,
        file_size INTEGER,
        PRIMARY KEY (identifier, file_name)
      )
    ''');

    // Create indexes for preview cache
    await db.execute('''
      CREATE INDEX idx_preview_cache_identifier 
      ON $tablePreviewCache(identifier)
    ''');

    await db.execute('''
      CREATE INDEX idx_preview_cache_cached_at 
      ON $tablePreviewCache(cached_at DESC)
    ''');

    await db.execute('''
      CREATE INDEX idx_preview_cache_type 
      ON $tablePreviewCache(preview_type)
    ''');

    debugPrint('Database schema created successfully');
  }

  /// Handle database upgrades
  Future<void> _onUpgrade(Database db, int oldVersion, int newVersion) async {
    debugPrint('Upgrading database from version $oldVersion to $newVersion');

    // Migration from version 1 to version 2: Add preview cache table
    if (oldVersion < 2) {
      debugPrint('Migrating to version 2: Adding preview_cache table');
      
      await db.execute('''
        CREATE TABLE $tablePreviewCache (
          identifier TEXT NOT NULL,
          file_name TEXT NOT NULL,
          preview_type TEXT NOT NULL,
          text_content TEXT,
          preview_data BLOB,
          cached_at INTEGER NOT NULL,
          file_size INTEGER,
          PRIMARY KEY (identifier, file_name)
        )
      ''');

      await db.execute('''
        CREATE INDEX idx_preview_cache_identifier 
        ON $tablePreviewCache(identifier)
      ''');

      await db.execute('''
        CREATE INDEX idx_preview_cache_cached_at 
        ON $tablePreviewCache(cached_at DESC)
      ''');

      await db.execute('''
        CREATE INDEX idx_preview_cache_type 
        ON $tablePreviewCache(preview_type)
      ''');

      debugPrint('Migration to version 2 completed successfully');
    }

    // Migration from version 2 to version 3: Add etag column to cached_metadata
    if (oldVersion < 3) {
      debugPrint('Migrating to version 3: Adding etag column');
      
      await db.execute('''
        ALTER TABLE $tableCachedMetadata ADD COLUMN etag TEXT
      ''');

      debugPrint('Migration to version 3 completed successfully');
    }
  }

  /// Close database connection
  Future<void> close() async {
    final db = await database;
    await db.close();
    _database = null;
    debugPrint('Database connection closed');
  }

  /// Delete database (useful for testing or reset)
  Future<void> deleteDatabase() async {
    try {
      final databasesPath = await getDatabasesPath();
      final path = join(databasesPath, _databaseName);
      await databaseFactory.deleteDatabase(path);
      _database = null;
      debugPrint('Database deleted successfully');
    } catch (e) {
      debugPrint('Error deleting database: $e');
      rethrow;
    }
  }

  /// Get database size in bytes
  Future<int> getDatabaseSize() async {
    try {
      // Since we can't easily get file size in Flutter without dart:io,
      // estimate based on row count
      final db = await database;
      
      // Count cached metadata
      final metadataResult = await db.rawQuery(
        'SELECT COUNT(*) as count FROM $tableCachedMetadata',
      );
      final metadataCount = Sqflite.firstIntValue(metadataResult) ?? 0;
      
      // Count preview cache and sum data sizes
      final previewResult = await db.rawQuery('''
        SELECT 
          SUM(LENGTH(text_content)) as text_size,
          SUM(LENGTH(preview_data)) as blob_size
        FROM $tablePreviewCache
      ''');
      final textSize = previewResult.first['text_size'] as int? ?? 0;
      final blobSize = previewResult.first['blob_size'] as int? ?? 0;
      
      // Rough estimate: ~50KB per cached archive + actual preview sizes
      return (metadataCount * 50 * 1024) + textSize + blobSize;
    } catch (e) {
      debugPrint('Error getting database size: $e');
      return 0;
    }
  }

  /// Vacuum database to reclaim space after deletions
  Future<void> vacuum() async {
    try {
      final db = await database;
      await db.execute('VACUUM');
      debugPrint('Database vacuumed successfully');
    } catch (e) {
      debugPrint('Error vacuuming database: $e');
    }
  }
}
