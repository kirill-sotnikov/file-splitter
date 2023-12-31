use std::{env::args, fs, collections::HashMap};

fn get_chunk_size(chunk_with_unit: String) -> Option<usize>{

    let sizes: HashMap<char, usize> = HashMap::from([
        ('b',1),
        ('k',1024),
        ('m',1024*1024),
        ('g',1024*1024*1024),
    ]);


    let unit = chunk_with_unit.chars().collect::<Vec<char>>()[0];
    let chunk_size = chunk_with_unit[1..].to_string().parse::<usize>().unwrap();

    match sizes.get(&unit) {
        Some(size) => Some(chunk_size * (*size)),
        None => {
            println!("please set size for chunk size\n\
            Example: file-splitter someFile.txt m12");
            None
        }
    }
}

fn main() {
    let command_line_args = args().collect::<Vec<String>>();

    if command_line_args.len() < 3 {
        println!("please start command with chunk size\n\
        Example: file-splitter someFile.txt 1000");
        panic!();
    }

    let splitted_file_path = command_line_args[1].as_str().split(".").take(2).collect::<Vec<&str>>();
    let file_name = splitted_file_path[0];
    let extension = splitted_file_path[1];
    let file = format!("{}.{}",file_name, extension);
    let dir_name = file_name.split(".").collect::<Vec<&str>>()[0];

    let chunk_size = get_chunk_size(command_line_args[2].clone()).unwrap();

    match fs::read(file) {
        Err(error) => println!("{error}"),
        Ok(data) => {
            if data.len() > 0 {
                fs::create_dir(dir_name).unwrap();

                for (file_chunk_index, file_chunk) in data.chunks(chunk_size).enumerate() {
                    match fs::write(
                        format!("{dir_name}/{file_name}_{file_chunk_index}.{extension}").as_str(),
                        file_chunk,
                    ) {
                        Err(error) => {
                            println!("{error}");
                            panic!()
                        }
                        Ok(_) => {
                            println!("part {file_chunk_index} ready!")
                        }
                    };
                }
            }

            println!("Success!");
        }
    }
}
