use std::{
    env::args,
    ffi::OsString,
    fs::{self},
};

fn get_index(file_name: &OsString) -> u32 {
    file_name
        .to_str()
        .unwrap()
        .to_string()
        .split("_")
        .last()
        .unwrap()
        .split(".")
        .take(1)
        .map(|i| i.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()[0]
}

fn main() {
    let command_line_args = args().collect::<Vec<String>>();
    let dir_name = command_line_args[1].as_str();
    let out_file_name = command_line_args[2].as_str();

    match fs::read_dir(dir_name) {
        Err(error) => {
            println!("{}", error);
            panic!();
        }
        Ok(data) => {
            let mut row_files: Vec<OsString> = vec![];
            let mut out_file_extension: &str;

            for i in data {
                let file_name = i.unwrap().file_name();
                out_file_extension = file_name
                    .to_str()
                    .unwrap()
                    .split(".")
                    .collect::<Vec<&str>>()[1];
                row_files.push(file_name);
            }

            row_files.sort_by(|prev, next| get_index(prev).cmp(&get_index(next)));

            println!("{:?}", row_files);

            let mut result_file: Vec<u8> = vec![];

            println!("Please wait...");

            for file_name in row_files {
                for i in fs::read(format!("{}{}", dir_name, file_name.to_str().unwrap())).unwrap() {
                    result_file.push(i);
                }
            }

            fs::write(out_file_name, result_file).unwrap();
        }
    }
}
