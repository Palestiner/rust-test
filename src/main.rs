#![windows_subsystem = "windows"]

use std::{thread, time};
use std::fs::{File, read_dir};
use std::io::{Read, Write};
use std::io;
use std::path::Path;

const CONF_NAME: &str = "app.txt";

fn main() -> io::Result<()> {
    // Чтение файла конфигурации
    let mut conf_file = open_file(Path::new(CONF_NAME));
    let conf_content = read_to_string(&mut conf_file);
    let split = conf_content.split("\n").collect::<Vec<&str>>();
    // если количество свойств не равно 3 завершаем программу
    if split.len() != 3 {
        panic!("Not enough properties in {}", CONF_NAME);
    }
    // обработка конфига
    let from_path = Path::new(split[0].trim());
    let to = split[1].trim();
    let min = &split[2].trim().parse::<u64>().expect("Third argument is not a number");
    // копирование файлов
    loop {
        for entry in read_dir(from_path)? {
            let file_path = entry?.path();
            let mut file = open_file(file_path.as_ref());
            let file_content = read_to_string(&mut file);
            let to_file = to.to_string() + &*file_path.file_name().unwrap().to_str().unwrap().to_string();
            copy_file(file_content.as_str(), Path::new(to_file.as_str()));
        }
        thread::sleep(time::Duration::from_millis(1000 * 60 * min));
    }
}

// чтение файла
fn open_file(path: &Path) -> File {
    let display = path.display();
    let result = match File::open(path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(result) => result
    };
    result
}

// контент в строку
fn read_to_string(file: &mut File) -> String {
    let mut strings_from_file = String::new();
    match file.read_to_string(&mut strings_from_file) {
        Err(why) => panic!("Couldn't read {:?}: {}", file, why),
        Ok(_) => println!("Correct read {:?}", file),
    };
    strings_from_file
}

// копирование в файла
fn copy_file(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;
    f.write_all(s.as_bytes())
}