use crate::settings::Settings;
use crate::utils::http::{parse_proxy, web_get_async};
use std::io;
use std::path::Path;

fn is_remote_path(path: &str) -> bool {
    path.starts_with("http://") || path.starts_with("https://")
}

pub async fn read_file(path: &str) -> Result<String, io::Error> {
    if !is_remote_path(path) {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            format!("local path is not supported in wasm runtime: {path}"),
        ));
    }

    let settings = Settings::current();
    let proxy = parse_proxy(&settings.proxy_config);
    let response = web_get_async(path, &proxy, None).await.map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("failed to fetch remote content: {}", e.message),
        )
    })?;

    if !(200..300).contains(&response.status) {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("http status {} while reading {path}", response.status),
        ));
    }

    Ok(response.body)
}

pub async fn read_file_async(path: &str) -> Result<String, io::Error> {
    read_file(path).await
}

pub async fn file_exists(path: &str) -> bool {
    if !is_remote_path(path) {
        return false;
    }

    let settings = Settings::current();
    let proxy = parse_proxy(&settings.proxy_config);
    match web_get_async(path, &proxy, None).await {
        Ok(response) => (200..300).contains(&response.status),
        Err(_) => false,
    }
}

async fn file_get<P: AsRef<Path>>(path: P, _base_path: Option<&str>) -> io::Result<String> {
    let path_str = path
        .as_ref()
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "path is not valid UTF-8"))?;
    read_file(path_str).await
}

pub async fn file_get_async<P: AsRef<Path>>(
    path: P,
    base_path: Option<&str>,
) -> io::Result<String> {
    file_get(path, base_path).await
}

pub async fn copy_file(_src: &str, _dst: &str) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "copy_file is not supported in wasm runtime",
    ))
}
