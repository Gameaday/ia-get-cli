import 'package:sqflite/sqflite.dart';
import 'package:path/path.dart';
import 'package:flutter/foundation.dart';

/// Database helper for managing SQLite database operations
/// Used for metadata caching with versioning and migrations
class DatabaseHelper {
  static const String _databaseName = 'ia_get.db';
  static const int _databaseVersion = 1;

  // Table names
  static const String tableCachedMetadata = 'cached_metadata';
  
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

    debugPrint('Database schema created successfully');
  }

  /// Handle database upgrades
  Future<void> _onUpgrade(Database db, int oldVersion, int newVersion) async {
    debugPrint('Upgrading database from version $oldVersion to $newVersion');

    // Migration logic for future schema changes
    if (oldVersion < 2) {
      // Example migration for version 2
      // await db.execute('ALTER TABLE ...');
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
      final databasesPath = await getDatabasesPath();
      final path = join(databasesPath, _databaseName);
      final file = await databaseFactory.getDatabasePath(path);
      // File size retrieval would require dart:io which may not be available in web
      // For now, return estimate based on row count
      final db = await database;
      final result = await db.rawQuery(
        'SELECT COUNT(*) as count FROM $tableCachedMetadata',
      );
      final count = Sqflite.firstIntValue(result) ?? 0;
      // Rough estimate: ~50KB per cached archive
      return count * 50 * 1024;
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
