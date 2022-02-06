use colored::*;
use std::env;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let base_txt_path = Path::new(&args[1]);

    let mut base_txtfile = match OpenOptions::new()
        .write(true)
        .read(true)
        .open(base_txt_path)
    {
        Ok(file) => file,
        Err(why) => panic!("Couldn't open: {}", why),
    };

    let mut contents = String::new();
    base_txtfile
        .read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    if contents.len() < 5000 {
        println!(
            "{}\nlength = {}",
            "Oops! This file seems to contain only less than 5000 characters.".bright_yellow(),
            contents.len().to_string().bold()
        );
        return;
    }

    let mut start_index = 0;
    let mut space_period_index = 0;
    let mut final_strings = String::new();
    for (index, value) in contents.chars().enumerate() {
        if value.to_string() == " " || value.to_string() == "." {
            space_period_index = index;
        }
        if index % 5000 == 0 && index != 0 {
            final_strings.push_str(contents.get(start_index..space_period_index).unwrap());
            final_strings.push_str("\n\n\n");
            start_index = space_period_index + 1;
        }
    }

    let mut output_txtfile = match OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(true)
        .open(format!(
            "output_{}",
            base_txt_path.file_name().unwrap().to_string_lossy()
        )) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't open: {}", why),
    };
    output_txtfile.write_all(final_strings.as_bytes()).unwrap();
    println!(
        "{}\n==> output_{}",
        "Success!!".bright_green().bold(),
        base_txt_path.file_name().unwrap().to_string_lossy()
    )
}
