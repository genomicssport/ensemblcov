use cmd_lib::run_cmd;
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
 Date: 2025-5-27
*/

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct FastaHuman {
    name: String,
    sequence: String,
    entirename: String,
}

static ADDRESS1:&str = "https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/release_48/gencode.v48.transcripts.fa.gz";
static ADDRESS2:&str = "https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/release_48/gencode.v48.pc_transcripts.fa.gz";

pub fn geneunwrap(ensemblid: &str) -> Result<String, Box<dyn Error>> {
    if ensemblid == "yes" {
        let _ = Command::new("wget")
            .arg(ADDRESS1)
            .output()
            .expect("command failed");
        let _ = Command::new("wget")
            .arg(ADDRESS2)
            .output()
            .expect("command failed or file not found");
        let _ = Command::new("gunzip")
            .arg("gencode.v48.pc_transcripts.fa.gz")
            .output()
            .expect("failed command");
        let _ = Command::new("gunzip")
            .arg("gencode.v48.transcripts.fa.gz")
            .output()
            .expect("failed command or file not found");
        let _ = run_cmd!(
         | awk r#"'/^>/ {printf("\n%s\n",$0);next; } { printf("%s",$0);} END {printf("\n");}'
                                              gencode.v48.transcripts.fa > gencode-transcripts.fa"#
        )
        .unwrap();
        let mut humanindex: Vec<FastaHuman> = Vec::new();
        let mut humanname: Vec<_> = Vec::new();
        let mut humanseq: Vec<_> = Vec::new();
        let mut humanentirename: Vec<_> = Vec::new();
        let fileread = File::open("gencode-transcripts.fa").expect("file not found");
        let filebuffer = BufReader::new(fileread);
        for i in filebuffer.lines() {
            let line = i.expect("line not present");
            if line.starts_with(">") {
                humanname.push(line.replace(">", "").replace("|", "-"));
                humanentirename.push(
                    line.replace(">", "")
                        .replace("|", "-")
                        .split("|")
                        .collect::<Vec<_>>()[0]
                        .to_string(),
                );
            } else {
                humanseq.push(line);
            }
        }
        for i in 0..humanname.len() {
            humanindex.push(FastaHuman {
                name: humanname[i].clone(),
                sequence: humanseq[i].clone(),
                entirename: humanentirename[i].clone(),
            });
        }
        let ensemblid = File::open(ensemblid).expect("file not present");
        let ensemblread = BufReader::new(ensemblid);
        let mut ensemblid_capture: Vec<_> = Vec::new();
        for i in ensemblread.lines() {
            let line = i.expect("line not present");
            ensemblid_capture.push(line);
        }
        let mut finalvector: Vec<FastaHuman> = Vec::new();
        for i in humanindex.iter() {
            for idval in ensemblid_capture.iter() {
                if idval.to_string() == i.entirename {
                    finalvector.push(FastaHuman {
                        name: i.name.clone(),
                        sequence: i.sequence.clone(),
                        entirename: i.entirename.clone(),
                    })
                }
            }
        }
        let mut finalfile = File::create("isolated-ids.fasta").expect("file not present");
        for i in finalvector.iter() {
            writeln!(finalfile, ">{}\n{}", i.entirename, i.sequence).expect("line not found");
        }
    }
    Ok("The gene list for the following ids have been written".to_string())
}
