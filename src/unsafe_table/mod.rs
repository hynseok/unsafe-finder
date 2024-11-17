use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use lazy_static::lazy_static;

lazy_static! {
    static ref UNSAFE_TABLE: HashMap<String, i32> = {
        let mut unsafe_table = HashMap::new();
        let file_path = "./unsafetab";
        let mut f = File::open(file_path).expect("Error: file not found");
        let mut buf = String::new();

        f.read_to_string(&mut buf).expect("Error: can not read file");
        for line in buf.split('\n') {
            let token: Vec<&str> = line.split_whitespace().collect();
            if token.len() >= 2 {
                unsafe_table.insert(token[0].to_string(), token[1].parse().unwrap());
            }
        }
        unsafe_table
    };
}

pub fn get_unsafe_table() -> &'static HashMap<String, i32> {
    &UNSAFE_TABLE
}