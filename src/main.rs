mod annotate;
mod args;
mod autogenerate;
mod counts;
mod differential;
use crate::annotate::generateannotate;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::autogenerate::generatecovid;
use crate::counts::convertcounts;
use crate::differential::differentialconvert;
use clap::Parser;

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-5-14
*/

fn main() {
    let argparse = CommandParse::parse();
    match &argparse.command {
        Commands::AutoGenerate { generate } => {
            let command = generatecovid(generate).unwrap();
            println!(
                "The gene conversion for the entire geneome has been written:{}",
                command
            );
        }
        Commands::GTFAnnotateGenerate { gtf } => {
            let command = generateannotate(gtf).unwrap();
            println!("The command has been completed:{}", command);
        }
        Commands::Countconvert { counts } => {
            let command = convertcounts(counts).unwrap();
            println!("The file conversion has been done:{}", command);
        }
        Commands::Differentialexpression {
            differntialexpression,
        } => {
            let command = differentialconvert(differntialexpression).unwrap();
            println!("The file has been converted:{}", command);
        }
    }
}
