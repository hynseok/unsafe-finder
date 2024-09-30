use std::{fs::File, io::Read};

pub fn get_file_ext(file_name: &String) -> &str {
    let token: Vec<&str> = file_name.split('.').collect();
    token[token.len() - 1]
}

pub fn get_unsafe_block(file_name: &String) -> String {
    let mut output = String::new();

    let mut f = File::open(file_name).expect("Error: file not found");

    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("Error: can not read file");

    let mut line_count = 1;
    let mut found = false;
    let mut stack = Vec::new();

    for line in buf.split('\n') {
        for token in line.split_whitespace() {
            match token {
                "unsafe" => {
                    if !found {
                        found = true;
                    }
                }
                _ => {
                    if found {
                        if token.contains('{') {
                            stack.push('{');
                        }
                        if token.contains('}') {
                            stack.pop();
                        }
                    }
                }
            }
        }

        if found {
            output.push_str(format!("{} | {}\n",line_count, line).as_str());
        }

        if found && stack.is_empty() {
            output.push('\n');
            found = false;
        }

        line_count += 1;
    }
    
    output
}
