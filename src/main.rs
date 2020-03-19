use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::{env, fs, io};

fn visit_dirs(dir: &Path, cb: &dyn Fn(&Path)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_name = entry.file_name().into_string().unwrap();

            if file_name.chars().nth(0) == Some('.') {
                continue;
            }
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&path);
            }
        }
    }
    Ok(())
}

fn file_count_line_callback(file_path: &Path) {
    let file = match File::open(&file_path) {
        Err(_) => return,
        Ok(file) => file,
    };

    let mut nb_line = 0;
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        match line {
            Err(_) => continue,
            Ok(_) => nb_line += 1,
        };
    }
    println!("file {:?} containt {} lines", file_path, nb_line);
    unsafe {
        NB_FILE_TOTAL += 1;
        NB_LINE_TOTAL += nb_line;
    }
}

static mut NB_FILE_TOTAL: u32 = 0;
static mut NB_LINE_TOTAL: u32 = 0;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        for idx in 1..args.len() {
            visit_dirs(args[idx].as_ref(), &file_count_line_callback);
        }
        println!("=============================");

        unsafe {
            println!("{} files containt {} lines", NB_FILE_TOTAL, NB_LINE_TOTAL);
        }
    } else {
        println!("Path missing");
    }
    Ok(())
}
