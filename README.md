# ensemblcov

- Complete set of multi-threaded genomics utilies for ensembl and differential expression matrix.
- Acknowledgements: MOSAIC platform, developed as part of the ECBiG-MOSAIC project (POIR.04.02.00-00-D017/20), co-financed by the European Regional Development Fund (ERDF) under the Smart Growth Operational Programme 2014-2020, Measure 4.2 for the development of modern research infrastructure in the science sector.
- Project PI and Informal queries: Prof. Luiza Handschuh: luizahan@ibch.poznan.pl.
- Code related queries: Dr. Gaurav Sablok: gsablok@ibch.poznan.pl.

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

- run this one time to generate annotation and it will store the converted annotation in a dotenv file.

```
- ensemblcov gtf-annotate-generate  sample.gtf
```

```
ensemblcov  autogenerate yes
ensemblcov countconvert ./sample-files/sample.matrix
ensemblcov differentialexpression ./sample-files/differntialresults.text
ensemblcov  gtf-annotate-generate ./sample-files/sample.gtf
```

Gaurav Sablok Instytut Chemii Bioorganicznej Polskiej Akademii Nauk ul. Noskowskiego 12/14 | 61-704, Poznań Poland
