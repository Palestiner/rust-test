#![windows_subsystem = "windows"]

use std::{thread, time};
use std::fs::{File, read_dir, copy};
use std::io::{Read, Write};
use std::io;
use std::path::Path;

const CONF_NAME: &str = "app.txt";

fn main() -> io::Result<()> {
    // Чтение файла конфигурации
    let mut conf_file = open_file(Path::new(CONF_NAME));
    let conf_content = read_to_string(&mut conf_file);
    let mut cow = String::from_utf8_lossy(conf_content.as_ref()).to_string();
    let split = cow.split("\n").collect::<Vec<&str>>();
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
            let to_file = to.to_string() + file_path.file_name().unwrap().to_str().unwrap();
            if file_path.is_file() {
                copy(file_path, Path::new(to_file.as_str()));
            }
        }
        panic!();
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
fn read_to_string(file: &mut File) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    match file.read_to_end(&mut vec) {
        Err(why) => panic!("Couldn't read {:?}: {}", file, why),
        Ok(_) => println!("Correct read {:?}", file),
    };
    vec
}

// копирование в файла
fn copy_file(vec: &Vec<u8>, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;
    f.write_all(vec)
}