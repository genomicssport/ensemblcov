use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "ensemblcov",
    version = "1.0",
    about = "human genomics utilities.
       ************************************************
      Gaurav Sablok, IBCH, PAN, Poznan, Poland,
      https://portal.ichb.pl/laboratory-of-genomics/.
      Email: gsablok@ibch.poznan.pl
      Prof. Luiza Handschuh
      Email: luizahan@ibch.poznan.pl.
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// threaded version of ensembl auto gene conversion
    ThreadedAuto {
        /// provide yes as argument
        generate: String,
    },
    /// autogenerate the ensemble gene conversion
    AutoGenerate {
        /// provide yes as argument
        generate: String
    },
    /// gtf file for annotation
    GTFAnnotateGenerate {
        /// path to the gtf file
        gtf: String,
    },
    /// id convert from counts file
    Countconvert {
        /// path to the counts matrix file
        counts: String,
    },
    /// id convert from differential expression
    Differentialexpression {
        /// path to the differential expression
        differntialexpression: String,
    },
    /// gene list extraction from ensembl
    GeneEnsembl {
        /// path to the ensembl ids
        ensemblid: String,
    },
    /// specific exons of the ensembl ids
    ExonEnsembl {
        /// path to the ensembl ids
        exonensembl: String,
    },
}
