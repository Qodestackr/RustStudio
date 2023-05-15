use std::collections::HashMap;

#[derive(Debug)]
struct File {
    name: String,
    content: String,
}

struct FileSystem {
    files: HashMap<String, File>,
}

impl FileSystem {
    fn new() -> FileSystem {
        FileSystem {
            files: HashMap::new(),
        }
    }

    fn create_file(&mut self, name: &str, content: &str) {
        let file = File {
            name: String::from(name),
            content: String::from(content),
        };
        self.files.insert(String::from(name), file);
    }

    fn read_file(&self, name: &str) -> Option<&File> {
        self.files.get(name)
    }

    fn delete_file(&mut self, name: &str) {
        self.files.remove(name);
    }
}

fn main() {
    let mut fs = FileSystem::new();

    fs.create_file("file1.txt", "This is file 1.");
    fs.create_file("file2.txt", "This is file 2.");

    let file1 = fs.read_file("file1.txt");
    println!("{:?}", file1);

    fs.delete_file("file2.txt");

    let file2 = fs.read_file("file2.txt");
    println!("{:?}", file2);
}
