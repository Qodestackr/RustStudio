use actix_web::{get, web, App, HttpResponse, HttpServer};
use std::collections::HashMap;

#[get("/{file}")]
async fn get_file(info: web::Path<(String,)>) -> HttpResponse {
    let file = info.into_inner().0;
    // Simulating file retrieval from the VFS
    let vfs_files = get_files_from_vfs();
    if let Some(file_contents) = vfs_files.get(&file) {
        HttpResponse::Ok().body(file_contents.clone())
    } else {
        HttpResponse::NotFound().body("File not found")
    }
}

fn get_files_from_vfs() -> HashMap<String, String> {
    // Simulated VFS data
    let mut vfs_files = HashMap::new();
    vfs_files.insert("file1.txt".to_owned(), "This is file 1".to_owned());
    vfs_files.insert("file2.txt".to_owned(), "This is file 2".to_owned());
    vfs_files.insert("file3.txt".to_owned(), "This is file 3".to_owned());
    vfs_files
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(get_file)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
