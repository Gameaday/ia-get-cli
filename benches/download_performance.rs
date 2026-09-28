use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::time::Duration;

use ia_get::core::session::{ArchiveFile, ArchiveMetadata};

/// Mock archive metadata for benchmarking
fn create_mock_metadata(file_count: usize, file_size: u64) -> ArchiveMetadata {
    let files: Vec<ArchiveFile> = (0..file_count)
        .map(|i| ArchiveFile {
            name: format!("test_file_{}.txt", i),
            source: "original".to_string(),
            format: Some("txt".to_string()),
            mtime: Some(1234567890),
            size: Some(file_size),
            md5: Some(format!("fake_md5_hash_{}", i)),
            crc32: None,
            sha1: None,
            btih: None,
            summation: None,
            original: None,
            rotation: None,
        })
        .collect();

    ArchiveMetadata {
        created: 1234567890,
        d1: "ia801234.us.archive.org".to_string(),
        d2: "ia901234.us.archive.org".to_string(),
        dir: "/01/items/benchmark_test".to_string(),
        files,
        files_count: file_count as u32,
        item_last_updated: 1234567890,
        item_size: file_size * file_count as u64,
        metadata: serde_json::json!({
            "identifier": "benchmark_test",
            "title": "Benchmark Test Archive"
        }),
        server: "ia801234.us.archive.org".to_string(),
        uniq: 123456789,
        workable_servers: vec!["ia801234.us.archive.org".to_string()],
        reviews: vec![],
    }
}

/// Benchmark metadata processing performance
fn bench_metadata_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("metadata_processing");

    for file_count in [10, 50, 100, 500, 1000].iter() {
        let metadata = create_mock_metadata(*file_count, 1024 * 1024); // 1MB files

        group.bench_with_input(
            BenchmarkId::new("file_count", file_count),
            &metadata,
            |b, metadata| {
                b.iter(|| {
                    // Summarize the metadata (file count + total size) - representative
                    // of the per-item work the downloader performs before scheduling files.
                    let total: u64 = metadata.files.iter().map(|f| f.size.unwrap_or(0)).sum();
                    black_box((metadata.files.len(), total));
                });
            },
        );
    }
    group.finish();
}

/// Benchmark size parsing operations
fn bench_size_parsing(c: &mut Criterion) {
    use ia_get::utilities::filters::{format_size, parse_size_string};

    let mut group = c.benchmark_group("size_parsing");

    let test_sizes = ["100MB", "2.5GB", "500KB", "1.2TB", "50B"];

    for (i, size_str) in test_sizes.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("parse_size", i),
            size_str,
            |b, &size_str| {
                b.iter(|| {
                    let result = parse_size_string(black_box(size_str));
                    let _ = black_box(result);
                });
            },
        );
    }

    let test_bytes = [1024, 1024 * 1024, 1024 * 1024 * 1024, 500, 2048];

    for (i, &bytes) in test_bytes.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("format_size", i), &bytes, |b, &bytes| {
            b.iter(|| {
                let result = format_size(black_box(bytes));
                black_box(result);
            });
        });
    }

    group.finish();
}

/// Benchmark URL processing operations
fn bench_url_processing(c: &mut Criterion) {
    use ia_get::utilities::common::{extract_identifier_from_url, validate_and_process_url};

    let mut group = c.benchmark_group("url_processing");

    let test_urls = [
        "https://archive.org/details/test-archive",
        "https://archive.org/details/another-test-archive-with-longer-name",
        "test-identifier",
        "https://archive.org/details/short",
    ];

    for (i, url) in test_urls.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("url_type", i), url, |b, &url| {
            b.iter(|| {
                let result = validate_and_process_url(black_box(url));
                let _ = black_box(result);
            });
        });

        group.bench_with_input(
            BenchmarkId::new("identifier_extraction", i),
            url,
            |b, &url| {
                b.iter(|| {
                    let result = extract_identifier_from_url(black_box(url));
                    let _ = black_box(result);
                });
            },
        );
    }
    group.finish();
}

/// Memory usage benchmark for large file operations
fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    group.measurement_time(Duration::from_secs(10));

    // Test with different numbers of files to simulate memory pressure
    for file_count in [1000, 5000, 10000].iter() {
        let metadata = create_mock_metadata(*file_count, 1024 * 1024);

        group.bench_with_input(
            BenchmarkId::new("large_metadata", file_count),
            &metadata,
            |b, metadata| {
                b.iter(|| {
                    // Simulate selecting files to download from large metadata
                    let files_to_download: Vec<String> = metadata
                        .files
                        .iter()
                        .take(100) // Only take first 100 to avoid actual downloads
                        .map(|f| f.name.clone())
                        .collect();

                    black_box(files_to_download);
                });
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_metadata_processing,
    bench_url_processing,
    bench_size_parsing,
    bench_memory_usage
);
criterion_main!(benches);
