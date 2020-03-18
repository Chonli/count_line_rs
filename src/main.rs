use std::{env, fs, io};

fn main() -> io::Result<()>  {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        for idx in 1..args.len() {
            let arg = args[idx].clone();
            println!("{}", arg);
            for entry in fs::read_dir(arg)? {
                let entry = entry?;
                let path = entry.path();
                println!("{:?}", path);
            }
        }
    } else {
        println!("Path missing");
    }
    Ok(())
}
