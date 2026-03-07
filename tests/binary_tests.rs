use alerter_rs::binary::{ALERTER_BINARY, extract_binary, get_binary_path};
use std::os::unix::fs::PermissionsExt;

#[test]
fn embedded_binary_constant_exists_and_is_non_empty() {
    assert!(
        !ALERTER_BINARY.is_empty(),
        "ALERTER_BINARY should contain the embedded alerter executable bytes"
    );
}

#[test]
fn embedded_binary_starts_with_macho_magic() {
    // Mach-O binaries start with magic bytes 0xFEEDFACE (32-bit) or 0xFEEDFACF (64-bit)
    // or the fat binary magic 0xCAFEBABE / 0xBEBAFECA
    assert!(
        ALERTER_BINARY.len() >= 4,
        "Binary must be at least 4 bytes for magic number"
    );
    let magic = u32::from_be_bytes([
        ALERTER_BINARY[0],
        ALERTER_BINARY[1],
        ALERTER_BINARY[2],
        ALERTER_BINARY[3],
    ]);
    let valid_magics = [
        0xFEEDFACE, // Mach-O 32-bit
        0xFEEDFACF, // Mach-O 64-bit
        0xCAFEBABE, // Fat binary
        0xBEBAFECA, // Fat binary (reversed)
        0xCFFAEDFE, // Mach-O 64-bit (little-endian)
        0xCEFAEDFE, // Mach-O 32-bit (little-endian)
    ];
    assert!(
        valid_magics.contains(&magic),
        "Embedded binary should start with a valid Mach-O magic number, got: {magic:#X}"
    );
}

#[test]
fn extract_binary_creates_executable_file() {
    let path = extract_binary().expect("extract_binary should succeed");
    assert!(path.exists(), "Extracted binary should exist on disk");

    let metadata = std::fs::metadata(&path).expect("Should be able to read file metadata");
    let permissions = metadata.permissions();
    assert!(
        permissions.mode() & 0o111 != 0,
        "Extracted binary should be executable"
    );
}

#[test]
fn extract_binary_returns_consistent_path() {
    let path1 = extract_binary().expect("First extraction should succeed");
    let path2 = extract_binary().expect("Second extraction should succeed");
    assert_eq!(
        path1, path2,
        "Subsequent calls to extract_binary should return the same path"
    );
}

#[test]
fn get_binary_path_returns_cached_location() {
    // Ensure extraction has happened first
    let extracted = extract_binary().expect("extract_binary should succeed");
    let cached = get_binary_path().expect("get_binary_path should succeed");
    assert_eq!(
        extracted, cached,
        "get_binary_path should return the same path as extract_binary"
    );
}

#[test]
fn cached_binary_is_reused_without_re_extraction() {
    let path = extract_binary().expect("First extraction should succeed");
    let modified_before = std::fs::metadata(&path)
        .expect("metadata")
        .modified()
        .expect("modified time");

    // Small sleep to ensure filesystem time granularity
    std::thread::sleep(std::time::Duration::from_millis(50));

    let path2 = extract_binary().expect("Second extraction should succeed");
    let modified_after = std::fs::metadata(&path2)
        .expect("metadata")
        .modified()
        .expect("modified time");

    assert_eq!(
        modified_before, modified_after,
        "Cached binary should not be re-written on subsequent calls"
    );
}

#[test]
fn extract_binary_path_is_in_cache_directory() {
    let path = extract_binary().expect("extract_binary should succeed");
    let path_str = path.to_string_lossy();
    assert!(
        path_str.contains("alerter_rs") || path_str.contains("alerter-rs"),
        "Binary should be extracted to an alerter_rs cache directory, got: {path_str}"
    );
}
