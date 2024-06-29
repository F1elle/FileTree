use std::fs;
use std::os::linux::fs::MetadataExt;

#[derive(Clone)]
pub struct TreeElement {
    path: String,
    is_dir: bool,
    children: Option<Vec<TreeElement>>,
    size_in_bytes: u128
}

impl TreeElement {
    pub fn new(path: String) -> Self {
        let is_dir = Self::is_element_dir(&path);
        let children = if is_dir {
            Some(Self::get_children(&path))
        } else {
            None
        };
        let size_in_bytes: u128 = {
            let mut size: u128 = 0;
            match &children {
                Some(vector) => {
                    for child in vector {
                        size+=child.size_in_bytes;
                    }
                },
                None => {
                    match fs::metadata(&path) {
                        Err(e) => {
                            size += 0u128;
                        },
                        Ok(metadata) => {
                            size += metadata.st_size() as u128;
                        }
                    }
                }
            };
            size
        };

        TreeElement {
            path: path.clone(),
            is_dir,
            children,
            size_in_bytes
        }
    }
    fn is_element_dir(path: &String) -> bool {
        match fs::metadata(&path) {
            Err(e) => { false },
            Ok( metadata) => {
                metadata.file_type().is_dir()
            }
        }
    }
    fn get_children(path: &String) -> Vec<TreeElement>{ // !!!!! this bullshit gotta be refactored
        // думаю буду передавать ошибку выше
        // и сообщать об ошибке в гуе
        let mut children: Vec<TreeElement> = vec!();
        let children_paths = fs::read_dir(path)
            .expect("Unable to read children dirs:?");
        for entry in children_paths {
            let path = match entry {
                Ok( entry ) => {
                    match entry.path().to_str() {
                        Some( path_str) => { path_str.to_string() },
                        None => panic!( "Can't convert this path to a string" ) // !!!!! DO SOMETHING HERE !!!!!
                    }
                },
                Err(e) => { panic!( "Can't read entry. {e}" ) } // !!!!! AND HERE !!!!!
            };
            children.push(Self::new(path));
        }
        children
    }
    pub fn get_element_size_string(&self) -> String {
        match self.size_in_bytes {
            0..=1023 => { format!("{} B", self.size_in_bytes) },
            1024..=1048575 => {
                format!("{size:.1} KB", size = self.size_in_bytes as f64 / 1024f64)
            },
            1048576..=1073741824 => {
                format!("{size:.1} MB", size = self.size_in_bytes as f64 / (1024*1024) as f64)
            },
            1073741825..=1099511627775 => {
                format!("{size:.1} GB", size = self.size_in_bytes as f64 / (1024*1024*1024) as f64)
            },
            _ => {String::from("42")}
        }
    }

    pub fn print_sorted_tree(&self) {
        let size_string = self.get_element_size_string();
        let mut sorted_children = self.children.clone();
        if let Some(mut children) = sorted_children {
            children.sort_by(|a, b| b.size_in_bytes.cmp(&a.size_in_bytes));
            println!(" {} {}", self.path, size_string);
            for child in children {
                println!("\t{} {}", child.path, child.get_element_size_string())
            }
        } else {
            println!("There are no files or folders...")
        }
    }
}