use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    let args : Vec<String> =env::args().collect();
    let var = &args[1];
    fs::remove_file(var)?;
    println! ("{} has been deleted", var);
    Ok(())
}
