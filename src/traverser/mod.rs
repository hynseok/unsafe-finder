use walkdir::WalkDir;

use crate::parser::{get_file_ext, get_unsafe_block};

pub fn traverse_dir(dir_name: String) {
    for entry in WalkDir::new(format!("../{}", dir_name))
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        
        match get_file_ext(&f_name) {
            "rs" => {
                let path = String::from(entry.into_path().to_string_lossy());
                let result = get_unsafe_block(&path);

                if result.len() != 0 {
                    println!("File path: {}", path.trim_start_matches("../"));
                    println!("{}", result);
                }
            }
            _ => (),
        }
    }
}
