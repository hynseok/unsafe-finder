use walkdir::WalkDir;

use crate::parser::{get_file_ext, get_unsafe_block};

pub struct UnsafeLine {
    file_name: String,
    line: i32,
    weight: i32,
}

pub fn traverse_dir(dir_name: String) {
    let mut output: Vec<UnsafeLine> = Vec::new();
    
    for entry in WalkDir::new(format!("./{}", dir_name))
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        
        match get_file_ext(&f_name) {
            "rs" => {
                let path = String::from(entry.into_path().to_string_lossy());
                let result = get_unsafe_block(&path);
                // let res_output = result.0;
                let res_candidates = result.1;

                // if res_output.len() != 0 {
                //     let trimmed_path = path.trim_start_matches("./");
                //     println!("File path: {}", trimmed_path);
                //     println!("{}", res_output);
                // }

                if res_candidates.len() != 0 {
                    for (line, weight) in res_candidates {
                        output.push(UnsafeLine {
                            file_name: f_name.clone(),
                            line,
                            weight,
                        });
                    }
                }
            }
            _ => (),
        }
    }

    output.sort_by(|a, b| b.weight.cmp(&a.weight));
    for line in output {
        println!("{}:{}", line.file_name, line.line);
    }
}
