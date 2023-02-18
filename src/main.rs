use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter,Write};




fn write_file(filename: &str, contents: &str) -> std::io::Result<()> {
    let mut f = File::create(filename)?;
    f.write_all(contents.as_bytes())?;
    Ok(())
}

fn read_file(filename: &str) -> std::io::Result<String> {
    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok("SUCCESS".to_string())
}

fn write_file_buf(filename: &str, contents: &str) -> std::io::Result<()> {
    let mut f = BufWriter::new(File::create(filename)?);
    f.write(&contents.as_bytes())?;

    Ok(())
}
    
fn read_file_buf(filename: &str) -> std::io::Result<String> {    									   let mut f = BufReader::new(File::open(filename)?);   let mut contents = String::new();  
    let mut f = BufReader::new(File::open(filename)?);
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;   
    Ok("SUCCESS".to_string()) 
}


fn main() -> std::io::Result<()> {

    let first_contents = "Cộng đồng Rust Việt Nam !";
    let second_contents = "Cộng đồng những người yêu thích ngôn ngữ lập trình Rust";

    write_file("./src/file/file.txt", first_contents)?;
    read_file("./src/file/file.txt")?;

    write_file_buf("./src/file/bytebuffer.txt", second_contents)?;
    read_file_buf("./src/file/bytebuffer.txt")?;

    Ok(())
}
