use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;
use std::thread;
/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-5-27
*/

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Exoncapture {
    name: String,
    exonstart: Vec<usize>,
    exonend: Vec<usize>,
}

static ADDRESS: &str = "https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/release_48/gencode.v48.chr_patch_hapl_scaff.annotation.gtf.gz";
static ADDRESS1: &str = "gencode.v48.chr_patch_hapl_scaff.annotation.gtf";

pub fn exonunwrap(exonlist: &str) -> Result<String, Box<dyn Error>> {
    let pathfile1 = Path::new(ADDRESS);
    let pathfile2 = Path::new(ADDRESS1);
    if !pathfile1.exists() && !pathfile2.exists() {
        thread::scope(|scope| {
            scope.spawn(|| {
                let _commandexec = Command::new("wget")
                    .arg(ADDRESS)
                    .output()
                    .expect("command failed");
                let _unzipcommand = Command::new("gunzip")
                    .arg("gencode.v48.chr_patch_hapl_scaff.annotation.gtf.gz")
                    .output()
                    .expect("command failed");
                let fileopen = File::open(ADDRESS1).expect("file not present");
                let fileread = BufReader::new(fileopen);
                let mut ensemblhashset: HashSet<_> = HashSet::new();
                let mut lineveccollect: Vec<_> = Vec::new();
                for i in fileread.lines() {
                    let hashline = i.expect("line not present");
                    if !hashline.starts_with("#") {
                        let hashvec: String = hashline.split("\t").collect::<Vec<_>>()[8]
                            .split(";")
                            .collect::<Vec<_>>()[0]
                            .split(" ")
                            .collect::<Vec<_>>()[1]
                            .to_string();
                        ensemblhashset.insert(hashvec.replace("\"", ""));
                        lineveccollect.push(hashline);
                    }
                }
                let mut exonvec: Vec<Exoncapture> = Vec::new();
                for i in ensemblhashset.iter() {
                    let mut exonst: Vec<usize> = Vec::new();
                    let mut exoned: Vec<usize> = Vec::new();
                    for val in lineveccollect.iter() {
                        if *i
                            == val.split("\t").collect::<Vec<_>>()[8]
                                .split(";")
                                .collect::<Vec<_>>()[0]
                                .split(" ")
                                .collect::<Vec<_>>()[1]
                                .to_string()
                                .replace("\"", "")
                            && val.split("\t").collect::<Vec<_>>()[2].to_string() == "exon"
                        {
                            exonst.push(
                                val.split("\t").collect::<Vec<_>>()[3]
                                    .parse::<usize>()
                                    .unwrap(),
                            );
                            exoned.push(
                                val.split("\t").collect::<Vec<_>>()[4]
                                    .parse::<usize>()
                                    .unwrap(),
                            );
                        }
                    }
                    exonvec.push(Exoncapture {
                        name: i.clone(),
                        exonstart: exonst,
                        exonend: exoned,
                    });
                }
                let exonids = File::open(exonlist).expect("File not present");
                let mut exonstorage: Vec<_> = Vec::new();
                let exonread = BufReader::new(exonids);
                for i in exonread.lines() {
                    let exonline = i.expect("line not present");
                    exonstorage.push(exonline);
                }

                let mut filewrite = File::create("selectedexons.txt").expect("file not present");
                for i in exonvec.iter() {
                    for val in exonstorage.iter() {
                        if i.name.to_string() == val.to_string() {
                            writeln!(
                                filewrite,
                                "{}\t{}\t{}",
                                i.name,
                                i.exonstart
                                    .clone()
                                    .into_iter()
                                    .map(|x| x.to_string())
                                    .collect::<Vec<_>>()
                                    .join("|"),
                                i.exonend
                                    .clone()
                                    .into_iter()
                                    .map(|x| x.to_string())
                                    .collect::<Vec<_>>()
                                    .join("|")
                            )
                            .expect("file not found");
                        }
                    }
                }
            });
        });
    }
    Ok("The file has been written with the following ids".to_string())
}
