use std::fs;
use std::io;
use std::io::Read;
use std::io::BufReader;
use std::env;
use crc::{Crc,CRC_16_XMODEM}; 


pub const CRCXMODEM:Crc<u16> = Crc::<u16>::new(&CRC_16_XMODEM); 

fn add_CRC(path:String){

    let f = fs::File::open(path.clone().trim()).expect("Error Reading");
    
    let mut reader= BufReader::new(f);
    let mut buffer:Vec<u8> = Vec::new();
    
    //Calculate CRC 
    reader.read_to_end(&mut buffer).expect("Error reading buffer"); 
    let mut digest = CRCXMODEM.digest(); 
    digest.update(&buffer);
    let mut crc_calc = digest.finalize().to_le_bytes().to_vec();
    
    //append buffer
    let mut jump_line:Vec<u8> = [0xA].to_vec();  
    buffer.append(&mut jump_line); 
    buffer.append(&mut crc_calc);  
 
    // add to the end 
    fs::write(path.trim(), buffer).expect("Error writing"); 
    // Sai is ok 
    println!("Arquivo {} teve seu xmodem CRC adicionado",path.trim());
    
}


fn main() {


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
}
