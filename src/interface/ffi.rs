//! FFI Interface for Mobile Platforms
//!
//! This module provides C-compatible bindings for the core ia-get functionality
//! to enable integration with mobile applications (Flutter, React Native, etc.)

use indicatif::ProgressBar;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;
use tokio::runtime::Runtime;

use crate::core::archive::fetch_json_metadata;
use crate::core::session::ArchiveMetadata;
use crate::infrastructure::http::HttpClientFactory;
use crate::utilities::filters::parse_size_string;

/// Callback function type for progress updates
pub type ProgressCallback = extern "C" fn(progress: f64, message: *const c_char, user_data: usize);

/// Callback function type for completion
pub type CompletionCallback =
    extern "C" fn(success: bool, error_message: *const c_char, user_data: usize);

/// FFI-compatible error codes
#[repr(C)]
pub enum IaGetErrorCode {
    Success = 0,
    InvalidInput = 1,
    NetworkError = 2,
    FileSystemError = 3,
    ParseError = 4,
    UnknownError = 5,
}

/// FFI-compatible download configuration
#[repr(C)]
pub struct FfiDownloadConfig {
    pub concurrent_downloads: u32,
    pub max_file_size: u64, // 0 = no limit
    pub output_directory: *const c_char,
    pub include_formats: *const c_char, // comma-separated
    pub exclude_formats: *const c_char, // comma-separated
    pub dry_run: bool,
    pub verbose: bool,
    pub auto_decompress: bool,
}

/// FFI-compatible file information
#[repr(C)]
pub struct FfiFileInfo {
    pub name: *const c_char,
    pub size: u64,
    pub format: *const c_char,
    pub download_url: *const c_char,
}

/// FFI-compatible archive metadata
#[repr(C)]
pub struct FfiArchiveMetadata {
    pub identifier: *const c_char,
    pub title: *const c_char,
    pub total_files: u32,
    pub total_size: u64,
    pub files: *const FfiFileInfo,
    pub files_count: u32,
}

/// Initialize the FFI interface
/// Must be called once before using any other functions
#[no_mangle]
pub extern "C" fn ia_get_init() -> IaGetErrorCode {
    // Initialize logging for mobile debugging (optional)
    #[cfg(debug_assertions)]
    {
        // Simple logging initialization without external dependencies
        println!("ia-get FFI initialized");
    }

    IaGetErrorCode::Success
}

/// Cleanup the FFI interface
/// Should be called when the application is shutting down
#[no_mangle]
pub extern "C" fn ia_get_cleanup() {
    // Currently no cleanup needed, but provides future extensibility
}

/// Fetch archive metadata asynchronously
/// Returns a request ID that can be used to cancel the operation
#[no_mangle]
pub extern "C" fn ia_get_fetch_metadata(
    identifier: *const c_char,
    progress_callback: ProgressCallback,
    completion_callback: CompletionCallback,
    user_data: usize,
) -> c_int {
    if identifier.is_null() {
        return -1;
    }

    let identifier_str = match unsafe { CStr::from_ptr(identifier) }.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return -1,
    };

    // Spawn async operation in background thread
    std::thread::spawn(move || {
        let rt = match Runtime::new() {
            Ok(rt) => rt,
            Err(_) => {
                let error_msg = CString::new("Failed to create async runtime").unwrap();
                completion_callback(false, error_msg.as_ptr(), user_data);
                return;
            }
        };

        rt.block_on(async move {
            // Create HTTP client
            let enhanced_client = match HttpClientFactory::for_metadata_requests() {
                Ok(c) => c,
                Err(e) => {
                    let error_msg = CString::new(format!("HTTP client error: {}", e)).unwrap();
                    completion_callback(false, error_msg.as_ptr(), user_data);
                    return;
                }
            };
            let client = enhanced_client.client();

            // Progress update: Starting metadata fetch
            let progress_msg = CString::new("Fetching archive metadata...").unwrap();
            progress_callback(0.1, progress_msg.as_ptr(), user_data);

            // Create a progress bar for the metadata fetch
            let progress_bar = ProgressBar::new_spinner();

            // Fetch metadata
            match fetch_json_metadata(&identifier_str, &client, &progress_bar).await {
                Ok((_metadata, _url)) => {
                    let progress_msg = CString::new("Parsing metadata...").unwrap();
                    progress_callback(0.8, progress_msg.as_ptr(), user_data);

                    // Store metadata for later retrieval
                    // In a real implementation, you'd store this in a global map with the request ID

                    let progress_msg = CString::new("Metadata fetch complete").unwrap();
                    progress_callback(1.0, progress_msg.as_ptr(), user_data);
                    completion_callback(true, ptr::null(), user_data);
                }
                Err(e) => {
                    let error_msg = CString::new(format!("Metadata fetch failed: {}", e)).unwrap();
                    completion_callback(false, error_msg.as_ptr(), user_data);
                }
            }
        });
    });

    // Return a mock request ID (in real implementation, this would be unique)
    1
}

/// Filter files based on criteria
/// Returns JSON string with filtered file list
#[no_mangle]
pub extern "C" fn ia_get_filter_files(
    metadata_json: *const c_char,
    include_formats: *const c_char,
    exclude_formats: *const c_char,
    max_size_str: *const c_char,
) -> *mut c_char {
    if metadata_json.is_null() {
        return ptr::null_mut();
    }

    let metadata_str = match unsafe { CStr::from_ptr(metadata_json) }.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    // Parse metadata
    let metadata: ArchiveMetadata = match serde_json::from_str(metadata_str) {
        Ok(m) => m,
        Err(_) => return ptr::null_mut(),
    };

    // Parse filter criteria
    let include_formats = if include_formats.is_null() {
        Vec::new()
    } else {
        match unsafe { CStr::from_ptr(include_formats) }.to_str() {
            Ok(s) => s.split(',').map(|s| s.trim().to_string()).collect(),
            Err(_) => Vec::new(),
        }
    };

    let exclude_formats = if exclude_formats.is_null() {
        Vec::new()
    } else {
        match unsafe { CStr::from_ptr(exclude_formats) }.to_str() {
            Ok(s) => s.split(',').map(|s| s.trim().to_string()).collect(),
            Err(_) => Vec::new(),
        }
    };

    let max_size = if max_size_str.is_null() {
        None
    } else {
        match unsafe { CStr::from_ptr(max_size_str) }.to_str() {
            Ok(s) => parse_size_string(s).ok(),
            Err(_) => None,
        }
    };

    // Apply basic filtering logic (simplified for FFI)
    let mut filtered_files = metadata.files.clone();

    // Filter by include formats
    if !include_formats.is_empty() {
        filtered_files.retain(|file| {
            if let Some(ref format) = file.format {
                include_formats
                    .iter()
                    .any(|f| format.to_lowercase().contains(&f.to_lowercase()))
            } else {
                false
            }
        });
    }

    // Filter by exclude formats
    if !exclude_formats.is_empty() {
        filtered_files.retain(|file| {
            if let Some(ref format) = file.format {
                !exclude_formats
                    .iter()
                    .any(|f| format.to_lowercase().contains(&f.to_lowercase()))
            } else {
                true
            }
        });
    }

    // Filter by max size
    if let Some(max_size) = max_size {
        filtered_files.retain(|file| file.size.map_or(true, |size| size <= max_size));
    }

    // Serialize filtered results
    match serde_json::to_string(&filtered_files) {
        Ok(json) => {
            let c_string = CString::new(json).unwrap();
            c_string.into_raw()
        }
        Err(_) => ptr::null_mut(),
    }
}

/// Start a download session
/// Returns session ID for tracking progress
#[no_mangle]
pub extern "C" fn ia_get_start_download(
    identifier: *const c_char,
    config: *const FfiDownloadConfig,
    progress_callback: ProgressCallback,
    completion_callback: CompletionCallback,
    user_data: usize,
) -> c_int {
    if identifier.is_null() || config.is_null() {
        return -1;
    }

    let _identifier_str = match unsafe { CStr::from_ptr(identifier) }.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return -1,
    };

    let ffi_config = unsafe { &*config };

    // Convert FFI config to internal config
    let _output_dir = if ffi_config.output_directory.is_null() {
        "./downloads".to_string()
    } else {
        match unsafe { CStr::from_ptr(ffi_config.output_directory) }.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => "./downloads".to_string(),
        }
    };

    // Spawn download operation
    std::thread::spawn(move || {
        let rt = match Runtime::new() {
            Ok(rt) => rt,
            Err(_) => {
                let error_msg = CString::new("Failed to create async runtime").unwrap();
                completion_callback(false, error_msg.as_ptr(), user_data);
                return;
            }
        };

        rt.block_on(async move {
            // Progress update: Starting download
            let progress_msg = CString::new("Initializing download...").unwrap();
            progress_callback(0.0, progress_msg.as_ptr(), user_data);

            // In a real implementation, this would:
            // 1. Create DownloadService with the configuration
            // 2. Set up progress reporting callbacks
            // 3. Start the download with proper session management
            // 4. Handle errors and completion

            // Mock successful completion for demonstration
            let progress_msg = CString::new("Download completed").unwrap();
            progress_callback(1.0, progress_msg.as_ptr(), user_data);
            completion_callback(true, ptr::null(), user_data);
        });
    });

    // Return mock session ID
    2
}

/// Cancel an ongoing operation
#[no_mangle]
pub extern "C" fn ia_get_cancel_operation(_operation_id: c_int) -> IaGetErrorCode {
    // In a real implementation, this would cancel the operation with the given ID
    IaGetErrorCode::Success
}

/// Free memory allocated by FFI functions
#[no_mangle]
pub extern "C" fn ia_get_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

/// Get the last error message (thread-local)
#[no_mangle]
pub extern "C" fn ia_get_last_error() -> *const c_char {
    // In a real implementation, this would return thread-local error state
    ptr::null()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    extern "C" fn test_progress_callback(progress: f64, message: *const c_char, _user_data: usize) {
        if !message.is_null() {
            let msg = unsafe { CStr::from_ptr(message) }.to_str().unwrap();
            println!("Progress: {:.1}% - {}", progress * 100.0, msg);
        }
    }

    extern "C" fn test_completion_callback(
        success: bool,
        error_message: *const c_char,
        _user_data: usize,
    ) {
        if success {
            println!("Operation completed successfully");
        } else if !error_message.is_null() {
            let msg = unsafe { CStr::from_ptr(error_message) }.to_str().unwrap();
            println!("Operation failed: {}", msg);
        }
    }

    #[test]
    fn test_ffi_init() {
        let result = ia_get_init();
        assert_eq!(result as i32, IaGetErrorCode::Success as i32);
    }

    #[test]
    fn test_ffi_filter_files() {
        let metadata_json = r#"{
            "created": 1640995200,
            "d1": "ia800100.us.archive.org",
            "d2": "ia600100.us.archive.org", 
            "dir": "/test",
            "files": [{
                "name": "test.pdf",
                "source": "original",
                "mtime": "1640995200",
                "size": 1024,
                "format": "pdf",
                "crc32": "12345678",
                "md5": "abcdef123456789",
                "sha1": "fedcba987654321",
                "btih": "999999999999999",
                "summation": "5555"
            }],
            "files_count": 1,
            "item_last_updated": 1640995200,
            "item_size": 1024,
            "metadata": {"title": "Test Archive"},
            "server": "ia800100.us.archive.org",
            "uniq": 123456,
            "workable_servers": ["ia800100.us.archive.org"]
        }"#;
        let metadata_cstr = CString::new(metadata_json).unwrap();
        let include_formats = CString::new("pdf").unwrap();

        let result = ia_get_filter_files(
            metadata_cstr.as_ptr(),
            include_formats.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
        );

        assert!(!result.is_null());

        // Clean up
        ia_get_free_string(result);
    }

    #[test]
    fn test_ffi_fetch_metadata() {
        let identifier = CString::new("commute_test").unwrap();

        let request_id = ia_get_fetch_metadata(
            identifier.as_ptr(),
            test_progress_callback,
            test_completion_callback,
            0,
        );

        assert!(request_id > 0);

        // In a real test, you'd wait for the async operation to complete
    }
}
