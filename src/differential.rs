use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
use std::env;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::error::Error;

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-5-26
*/

pub fn differentialconvert(pathfile: &str) -> Result<String, Box<dyn Error>>{
    Ok("The file conversion has been written".to_string())
}
