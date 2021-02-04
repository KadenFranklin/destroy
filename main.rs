use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    for arg in env::args().skip(1) {
        for entry in fs::read_dir(".")? {
            let dir = entry?;
            if dir.path().to_string_lossy() == arg {
                fs::remove_file(dir.path())?;
                println! ("{} has been deleted", arg);
            }
        }
    }
    Ok(())
}