use std::path::Path;
use std::{env, fs, io};

fn visit_dirs(dir: &Path, cb: &dyn Fn(&Path)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_name = entry.file_name().into_string().unwrap();

            if file_name.chars().next() == Some('.') {
                println!("hidden file {:?}", file_name);
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

fn file_callback(file: &Path) {
    println!("{}", file.display());
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        for idx in 1..args.len() {
            println!("{}", args[idx]);
            visit_dirs(args[idx].as_ref(), &file_callback);
        }
    } else {
        println!("Path missing");
    }
    Ok(())
}
