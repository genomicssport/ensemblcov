use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-5-27
*/

pub fn geneunwrap(ensemblid: &str) -> Result<String, Box<dyn Error>>{
    Ok("The gene list for the following ids have been written".to_string())
}
