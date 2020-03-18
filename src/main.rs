use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        for idx in 1..args.len() {
            println!("{}", args[idx]);
        }        
    }else{
        println!("Path missing");
    }
}
