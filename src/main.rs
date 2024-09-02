use std::fs;
use std::io::Read;
use std::io::BufReader;
use crc::{Crc,*}; 
use clap::Parser;

#[derive(Parser,Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The mode for the CRC calculation current developed
    /// 0 :XModem CRC (Default)
    /// 1 :CRC IBM 3740/n
    /// 2 :CRC GSM 16 bits
    /// 3 :CRC USB 16 bits
    /// 4 : CRC ATM 10 bits
    /// 5 : CRC 15 CAN
    mode: u32,
    /// The path to the file to append the CRC after newline 
    path: std::path::PathBuf,
    
    ////If print the CRC on the screen
    #[clap(long, short, action)]
    print: bool,
    
    //// Flag to write to the file provided in path 
    #[clap(long, short, action)]
    write:bool 
    
}


fn add_CRC(arguments:Cli)
{

    let f = fs::File::open(arguments.path.to_str().expect("Cannot deal with path string").clone().trim()).expect("Error Reading");
    
    let mut reader= BufReader::new(f);
    let mut buffer:Vec<u8> = Vec::new();
    
    //Calculate CRC 
    reader.read_to_end(&mut buffer).expect("Error reading buffer"); 
    let crc_obj = match arguments.mode {
        0 => Crc::<u16>::new(&CRC_16_XMODEM),
        1 => Crc::<u16>::new(&CRC_16_IBM_3740),
        2 => Crc::<u16>::new(&CRC_16_GSM), 
        3 => Crc::<u16>::new(&CRC_16_USB),
        4 => Crc::<u16>::new(&CRC_10_ATM),
        5 => Crc::<u16>::new(&CRC_15_CAN), 
        _ => Crc::<u16>::new(&CRC_16_XMODEM)

        // Need to find a way to look at all the CRC possibilities. 
    };

    let mut digest = crc_obj.digest();     
    digest.update(&buffer);

    let mut crc_calc = digest.finalize().to_le_bytes();
    //append buffer
    let mut jump_line:Vec<u8> = [0xA].to_vec();  
    buffer.append(&mut jump_line); 
    buffer.append(&mut crc_calc.to_vec());  
    
    // add to the end 
    if arguments.write
    {
    fs::write(arguments.path.to_str().expect("Cannot deal with path string on write mode file not appended CRC").trim(), buffer).expect("Error writing"); 
    println!("File {} CRC selected appended to newline in file",arguments.path.to_str().expect("Failure").trim());
    } 

    if arguments.print
    {
        println!(" Calculated CRC for the algorithm selected {:#04x}", u16::from_le_bytes(crc_calc));
    }
    
}


fn main() {

    let args = Cli::parse(); 
    

    add_CRC(args); 

    /* 
    let mut args : Vec<String> = env::args().collect();
    let mut path = String::new();
    println!("{:?}",args.len());
    if args.len()<2 {
    println!("This program adds CRC Xmodem to the end of a file"); 
    println!("Enter the path to the file");
    } 
    if args.len()<2{
    // read file 
    io::stdin().read_line(&mut path).expect("Something wento wrong with path");
    add_CRC(path.clone());
    }
    else {
    while args.len()>1{

        let path = &args[1];
        println!("{:?}",path.to_string().clone());
        add_CRC(path.to_string().clone());
        args.remove(1);   
    }
    } 
    */
}
