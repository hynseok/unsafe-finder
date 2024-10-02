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
    let mut f_def = String::new();
    let mut f_flag = 0;
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
                "fn" => {
                    if f_flag == 0 {
                        f_flag = 1;
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
                    if f_flag == 1 {
                        f_def = token.to_string();
                        f_flag = 2;
                    }
                }
            }
        }
        
        if found {
            if f_flag == 2 {
                let f_name: Vec<&str> = f_def.split('(').collect();
                output.push_str(format!("Function Name: {}\n", f_name[0]).as_str());
                f_flag = 0;
            }
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
