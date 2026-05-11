use std::fs;
use std::io::{self, Read};
use std::path::Path;

/// Read a file into a string
pub fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Async version of read_file that reads a file into a string asynchronously
///
/// # Arguments
/// * `path` - Path to the file to read
///
/// # Returns
/// * `Ok(String)` - The file contents
/// * `Err(io::Error)` - If the file can't be read
pub async fn read_file_async(path: &str) -> Result<String, io::Error> {
    tokio::fs::read_to_string(path).await
}

/// Check if a file exists
pub async fn file_exists(path: &str) -> bool {
    tokio::fs::metadata(path).await.is_ok()
}

/// Read the contents of a file as a string
///
/// # Arguments
/// * `path` - Path to the file to read
/// * `base_path` - Optional base path for security checking
///
/// # Returns
/// * `Ok(String)` - The file contents
/// * `Err(io::Error)` - If the file can't be read
pub fn file_get<P: AsRef<Path>>(path: P, base_path: Option<&str>) -> io::Result<String> {
    if let Some(base_path) = base_path {
        match path.as_ref().to_str() {
            Some(path_str) => {
                if !path_str.starts_with(base_path) {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "File path is not within the base path",
                    ));
                }
            }
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "File path is not a valid UTF-8 string",
                ));
            }
        }
    }
    fs::read_to_string(path)
}

/// Copy a file from source to destination
pub async fn copy_file(src: &str, dst: &str) -> io::Result<()> {
    // Check if source exists
    if !Path::new(src).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Source file {} not found", src),
        ));
    }

    // Create parent directories if they don't exist
    if let Some(parent) = Path::new(dst).parent() {
        fs::create_dir_all(parent)?;
    }

    // Copy the file
    fs::copy(src, dst)?;
    Ok(())
}

/// Async version of file_get that reads file contents asynchronously
///
/// # Arguments
/// * `path` - Path to the file to read
/// * `base_path` - Optional base path for security checking
///
/// # Returns
/// * `Ok(String)` - The file contents
/// * `Err(io::Error)` - If the file can't be read
pub async fn file_get_async<P: AsRef<Path>>(
    path: P,
    base_path: Option<&str>,
) -> io::Result<String> {
    if let Some(base_path) = base_path {
        match path.as_ref().to_str() {
            Some(path_str) => {
                if !path_str.starts_with(base_path) {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "File path is not within the base path",
                    ));
                }
            }
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "File path is not a valid UTF-8 string",
                ));
            }
        }
    }
    tokio::fs::read_to_string(path).await
}
