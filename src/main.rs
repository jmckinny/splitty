use clap::Parser;
use std::{
    fs::File,
    fs::create_dir,
    io::{Read, Write},
};
fn main() -> std::io::Result<()> {
    let args = Args::parse();
    println!("{:?}", args);
    let mut file = File::open(args.file)?;
    let file_size = file.metadata()?.len() as usize;
    splitter(&mut file,args.parts_dir.as_str() ,args.block_size, file_size);
    Ok(())
}

fn splitter(f: &mut File, parts_dir: &str,block_size: usize, max_size: usize) {
    let mut chunks = max_size / block_size;
    let remainder = max_size - (block_size * chunks);
    chunks = if remainder != 0 { chunks + 1 } else { chunks };
    let mut buffer = vec![0; block_size];
    create_dir(parts_dir).expect("Failed to create parts directory");
    for i in 1..=chunks {
        let bytes = f.read(&mut buffer).expect("Failed to read bytes");
        println!("Read chunk {} size {}", i, bytes);
        let mut output_file = File::create(format!("{}/part{}",parts_dir ,i)).expect("Failed to write to file");
        output_file
            .write_all(&buffer[0..bytes])
            .expect("Failed to write to file");
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to file to be split
    #[clap(short, long, value_parser)]
    file: String,
    
    /// Size of each block
    #[clap(short,long,value_parser = valid_block_size,default_value = "1024")]
    block_size: usize,

    /// Output directory for parts
    #[clap(short,long,value_parser,default_value = "parts")]
    parts_dir: String
}

fn valid_block_size(s: &str) -> Result<usize, String> {
    let num: usize = s.parse().or(Err("Invalid number"))?;
    if num == 0 {
        Err("Blocksize can't be zero".to_string())
    } else {
        Ok(num)
    }
}
