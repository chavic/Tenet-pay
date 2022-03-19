
use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::env::current_dir;

pub async fn index(_req: HttpRequest) -> Result<NamedFile> {
    let mut current_path = current_dir()?;
    current_path.pop();
    current_path.push("frontend/dist/index.html");
    let path = current_path;

    let file = NamedFile::open(path)?;
    Ok(file)
}
