awk '/^>/ {printf("\n%s\n",$0);next; } { printf("%s",$0);} END {printf("\n");}' gencode.v48.transcripts.fa >transcripts.fa
