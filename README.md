# ensemblcov

<img src="https://github.com/IBCHgenomic/ensemblcov/blob/main/ensemblcov.png" width="350" />

- Complete set of multi-threaded genomics utilities for ensembl and differential expression matrix.

![](https://github.com/IBCHgenomic/eVaiutilities/blob/main/logo.png)

```
human genomics utilities.
       ************************************************
      Gaurav Sablok, IBCH, PAN, Poznan, Poland,
      https://portal.ichb.pl/laboratory-of-genomics/.
      Email: gsablok@ibch.poznan.pl
      Prof. Luiza Handschuh
      Email: luizahan@ibch.poznan.pl.
      ************************************************

Usage: ensemblcov <COMMAND>

Commands:
  threaded-auto           threaded version of ensembl auto gene conversion
  auto-generate           autogenerate the ensemble gene conversion
  gtf-annotate-generate   gtf file for annotation
  countconvert            id convert from counts file
  differentialexpression  id convert from differential expression
  gene-ensembl            gene list extraction from ensembl
  exon-ensembl            specific exons of the ensembl ids
  help                    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

- before running the ensemlcov additional options, run one time,either autogenerate or threaded-generate to link the ensemblid to geneid.

```
ensemblcov  autogenerate yes
ensemblcov threaded-auto yes
ensemblcov countconvert ./sample-files/sample.matrix
ensemblcov differentialexpression ./sample-files/differntialresults.text
ensemblcov  gtf-annotate-generate ./sample-files/sample.gtf
ensemblcov gene-ensembl id.txt
ensemblcov exon-ensembl ./sample-files/idexon.txt
```

- To install windows version:
```
rustup component add llvm-tools
rustup target add x86_64-pc-windows-msvc
git clone https://github.com/IBCHgenomic/ensemblcov.git
cd ensemblcov
cargo xwin build --target x86_64-pc-windows-msvc
```

- Acknowledgements: MOSAIC platform, developed as part of the ECBiG-MOSAIC project (POIR.04.02.00-00-D017/20), co-financed by the European Regional Development Fund (ERDF) under the Smart Growth Operational Programme 2014-2020, Measure 4.2 for the development of modern research infrastructure in the science sector.
- Project PI and Informal queries: Prof. Luiza Handschuh: luizahan@ibch.poznan.pl.
- Code related queries: Dr. Gaurav Sablok: gsablok@ibch.poznan.pl.

Gaurav Sablok Instytut Chemii Bioorganicznej Polskiej Akademii Nauk ul. Noskowskiego 12/14 | 61-704, Poznań Poland
