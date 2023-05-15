use std::collections::HashMap;

struct VirtualFileSystem {
    files: HashMap<String, String>,
}

impl VirtualFileSystem {
    fn new() -> Self {
        VirtualFileSystem {
            files: HashMap::new(),
        }
    }

    fn create_file(&mut self, name: &str, content: &str) {
        self.files.insert(name.to_owned(), content.to_owned());
    }

    fn read_file(&self, name: &str) -> Option<&str> {
        self.files.get(name).map(|content| content.as_str())
    }

    fn delete_file(&mut self, name: &str) -> Option<String> {
        self.files.remove(name)
    }

    fn list_files(&self) -> Vec<&str> {
        self.files.keys().map(|key| key.as_str()).collect()
    }
}

fn main() {
    // Create a new virtual file system
    let mut vfs = VirtualFileSystem::new();

    // Create files in the VFS
    vfs.create_file("file1.txt", "This is the content of file 1");
    vfs.create_file("file2.txt", "Content of file 2");

    // Read files from the VFS
    if let Some(content) = vfs.read_file("file1.txt") {
        println!("Content of file1.txt: {}", content);
    }

    // List all files in the VFS
    let files = vfs.list_files();
    println!("Files in the VFS: {:?}", files);

    // Delete a file from the VFS
    let deleted_file = vfs.delete_file("file2.txt");
    if let Some(name) = deleted_file {
        println!("Deleted file: {}", name);
    }

    // List files after deletion
    let files = vfs.list_files();
    println!("Files in the VFS: {:?}", files);
}
