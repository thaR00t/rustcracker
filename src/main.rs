use sha1::Digest;
use sha2::{
    Sha256,
    Sha224,
    Sha384,
    Sha512,
    };
use clap::{
    arg,
    Parser,
    command, 
    };
use std::{
    error::Error,
    fs::File,
    io::{BufRead,BufReader}
};
use cfonts::{
    say, 
    Options, 
    Fonts 
    };


const SHA1_TYPE: &str = "sha1";
const SHA1_HEX_MAX_LENGTH: usize = 40;

const MD5_TYPE: &str = "md5";
const MD5_HEX_MAX_LENGTH: usize = 32;

const SHA256_TYPE: &str = "sha256";
const SHA256_HEX_MAX_LENGTH: usize = 64;

const SHA224_TYPE: &str = "sha224";
const SHA224_HEX_MAX_LENGTH: usize = 56;

const SHA384_TYPE: &str = "sha384";
const SHA384_HEX_MAX_LENGTH: usize = 96;

const SHA512_TYPE: &str = "sha512";
const SHA512_HEX_MAX_LENGTH: usize = 128;


#[derive(Debug,Parser)]
#[command(author, version = "0.1", about,long_about = String::from("AVAILABLE HASH:
[*] md5
[*] sha1    
[*] sha224
[*] sha256
[*] sha384
[*] sha512"))]
struct Cli{
    #[arg(short, long)]
    hash: String,
    #[arg(short, long)]
    wordlist: std::path::PathBuf,

    #[arg(short, long, default_value_t = SHA1_TYPE.to_string())]
    type_hash: String,
    //#[command(subcommand)]
	//list_hashes: HASHES
}
fn main()  ->Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    //creating the hash type var which contains the address of the type_hash var.
    let h_type = &args.type_hash;

    say(Options{
        text: String::from("rustcracker"),
        font: Fonts::FontTiny,
        ..Options::default()
    });

    println!("---------------------------------------------------------------------");
    println!("[*] Hash:    {}", &args.hash);
    println!("[*] Mode:    {}", &args.type_hash);
    println!("[*] Wordlist {:?}", &args.wordlist);
    println!("---------------------------------------------------------------------");
    //Handling different type of hashes
    if h_type == &SHA1_TYPE.to_string(){
        sha_1()
        .map_err(|err| println!("{:?}", err))
        .ok();
    }
    else if h_type == &MD5_TYPE.to_string(){
        md_5()
        .map_err(|err| println!("{:?}", err))
        .ok();
    }
    else if h_type == &SHA256_TYPE.to_string(){
        sha_256()
        .map_err(|err| println!("{:?}", err))
        .ok();
    }
    else if h_type == &SHA224_TYPE.to_string(){
        sha_224()
        .map_err(|err| println!("{:?}", err))
        .ok();
    }
    else if h_type == &SHA384_TYPE.to_string(){
        sha_384()
        .map_err(|err| println!("{:?}", err))
        .ok();
    }
    else if h_type == &SHA512_TYPE.to_string(){
        sha_512()
        .map_err(|err| println!("{:?}", err))
        .ok();
    }
    else{
        return Err("No hash type found.".into());
    }
    
    Ok(())
    
}




fn md_5() ->Result<(), Box<dyn Error>>{

    let args = Cli::parse();
    let hash = &args.hash;
    if hash.len() != MD5_HEX_MAX_LENGTH{
        return Err("Error: md5 must be 32 hex chars!".into());
    }
    let f = File::open(&args.wordlist)?;
    let reader = BufReader::new(f);
    for line in reader.lines(){
        let line = line?;
        let data_md5 = md5::Md5::digest(line.as_bytes());
        if hash == &hex::encode(data_md5){
            println!("Hash found [ {} ]", line);
            return Ok(());
        }
    
    }
    println!("No matches found for this hash.");
    Ok(())
}

fn sha_1() ->Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let hash = &args.hash;
    if hash.len() != SHA1_HEX_MAX_LENGTH{
        return Err("Error: sha1 must be 40 hex chars!".into());
    }

    let f = File::open(&args.wordlist)?;
    let reader = BufReader::new(f);
    for line in reader.lines(){
        let line = line?;
        let data_sha1 = sha1::Sha1::digest(line.as_bytes());
        if hash == &hex::encode(data_sha1){
            println!("Hash found: [ {} ]", line);
            return Ok(());
        }
    }
    println!("No matches found for this hash.");
    Ok(())
}



fn sha_224() ->Result<(), Box<dyn Error>>{
    let args = Cli::parse();
    let hash = &args.hash;
    if hash.len() != SHA224_HEX_MAX_LENGTH{
        return Err("Error: sha224 must be 56 hex chars!".into());
    }
    let f = File::open(&args.wordlist)?;
    let reader = BufReader::new(f);
    for line in reader.lines(){
        let line = line?;
        let data_sha256 = Sha224::digest(line.as_bytes());
        if hash == &hex::encode(data_sha256){
            println!("Hash found [ {} ]", line);
            return Ok(());
        }
    
    }
    println!("No matches found for this hash.");
    Ok(())
}

fn sha_256() ->Result<(), Box<dyn Error>>{
    let args = Cli::parse();
    let hash = &args.hash;
    if hash.len() != SHA256_HEX_MAX_LENGTH{
        return Err("Error: sha1 must be 64 hex chars!".into());
    }
    let f = File::open(&args.wordlist)?;
    let reader = BufReader::new(f);
    for line in reader.lines(){
        let line = line?;
        let data_sha256 = Sha256::digest(line.as_bytes());
        if hash == &hex::encode(data_sha256){
            println!("Hash found [ {} ]", line);
            return Ok(());
        }
    
    }
    println!("No matches found for this hash.");
    Ok(())
}

fn sha_384() ->Result<(), Box<dyn Error>>{
    let args = Cli::parse();
    let hash = &args.hash;
    if hash.len() != SHA384_HEX_MAX_LENGTH{
        return Err("Error: sha1 must be 96 hex chars!".into());
    }
    let f = File::open(&args.wordlist)?;
    let reader = BufReader::new(f);
    for line in reader.lines(){
        let line = line?;
        let data_sha256 = Sha384::digest(line.as_bytes());
        if hash == &hex::encode(data_sha256){
            println!("Hash found [ {} ]", line);
            return Ok(());
        }
    
    }
    println!("No matches found for this hash.");
    Ok(())
}

fn sha_512() ->Result<(), Box<dyn Error>>{
    let args = Cli::parse();
    let hash = &args.hash;
    if hash.len() != SHA512_HEX_MAX_LENGTH{
        return Err("Error: sha512 must be 128 hex chars!".into());
    }
    let f = File::open(&args.wordlist)?;
    let reader = BufReader::new(f);
    for line in reader.lines(){
        let line = line?;
        let data_sha256 = Sha512::digest(line.as_bytes());
        if hash == &hex::encode(data_sha256){
            println!("Hash found [ {} ]", line);
            return Ok(());
        }
    
    }
    println!("No matches found for this hash.");
    Ok(())
}
