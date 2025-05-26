use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::process::Command;

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-5-26
*/

pub fn generatecovid(generate: &str) -> Result<String, Box<dyn Error>> {
    if generate == "yes" {
        let _commandexec = Command::new("wget").arg("https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/release_48/gencode.v48.chr_patch_hapl_scaff.annotation.gtf.gz").output().expect("command failed");
        let _unzipcommand = Command::new("gunzip")
            .arg("gencode.v48.chr_patch_hapl_scaff.annotation.gtf.gz")
            .output()
            .expect("command failed");
        let fileopen = File::open("gencode.v48.chr_patch_hapl_scaff.annotation.gtf")
            .expect("file not present");
        let fileread = BufReader::new(fileopen);
        let mut ensemblid: Vec<(String, String)> = Vec::new();
        for i in fileread.lines() {
            let gtfline = i.expect("file not present");
            if !gtfline.starts_with("#") {
                let ensemblinside: Vec<_> = gtfline.split("\t").collect::<Vec<_>>();
                if ensemblinside[2].to_string() == "gene" {
                    let ensembline = ensemblinside[8]
                        .split(";")
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>();
                    let ensembl_tuple = ensembline[0].split(" ").collect::<Vec<_>>();
                    let geneid_tuple = ensembline[2].split(" ").collect::<Vec<_>>();
                    let ensemblpush: (String, String) = (
                        ensembl_tuple[1].replace("\"", "").to_string(),
                        geneid_tuple[2].to_string().replace("\"", ""),
                    );
                    ensemblid.push(ensemblpush);
                }
            }
            let mut filewrite = File::create("annotation").expect("file not present");
            for i in ensemblid.iter() {
                writeln!(filewrite, "{},{}", i.0.to_string(), i.1.to_string())
                    .expect("file not present");
            }
        }
    }
    Ok("The file has been converted".to_string())
}
