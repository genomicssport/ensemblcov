use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::thread;

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-5-26
*/

pub fn differentialconvert(pathfile: &str) -> Result<String, Box<dyn Error>> {
    let countread = File::open(pathfile).expect("annotation file not present");
    let countread_final = BufReader::new(countread);
    let mut countread_ensembl: Vec<_> = Vec::new();
    let mut countrest_file: Vec<String> = Vec::new();
    for val in countread_final.lines() {
        let count = val.expect("line not present");
        let countvec = count.split("\t").map(|x| x.to_string()).collect::<Vec<_>>();
        countread_ensembl.push(countvec[0].clone());
        let restfile = countvec[1..countvec.len()].join("   ").to_string();
        countrest_file.push(restfile);
    }

    let annotationfile = File::open("annotation").expect("file not present");
    let annotationread = BufReader::new(annotationfile);
    let mut ensemblvec: Vec<String> = Vec::new();
    let mut geneidvec: Vec<String> = Vec::new();
    for i in annotationread.lines() {
        let line = i.expect("line not present");
        let annotationvec: Vec<_> = line.split(",").collect::<Vec<_>>();
        ensemblvec.push(annotationvec[0].to_string());
        geneidvec.push(annotationvec[1].to_string());
    }

    let mut finalcount_compare: Vec<(String, String, String)> = Vec::new();
    for i in 0..countread_ensembl.len() {
        for val in 0..ensemblvec.len() {
            if countread_ensembl[i].to_string() == ensemblvec[val].to_string() {
                let iter_tuple: (String, String, String) = (
                    countread_ensembl[i].to_string(),
                    geneidvec[val].to_string(),
                    countrest_file[val].to_string(),
                );
                finalcount_compare.push(iter_tuple);
            } else {
                continue;
            }
        }
    }

    let mut fileconverted = File::create("diffconverted.txt").expect("file not present");
    for i in finalcount_compare.iter() {
        writeln!(fileconverted, "{}\t{}\t{}", i.0, i.1, i.2).expect("file not present");
    }

    Ok("The counts file conversion has been written".to_string())
}
