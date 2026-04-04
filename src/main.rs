use std::fs;


fn main() {
  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    println!("usage: {} FILE", args[0]);
    return
  }

  let fs_read = fs::read_to_string(&args[1]);

  match fs_read {
    Ok(src) => println!("{}", src),
    Err(_) => println!("Could not read file {}", &args[1]),
  }
}
