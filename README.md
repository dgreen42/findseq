findseq

A cli tool that allows you to search a fasta (or related file type i.e .fna) for a sequence. 

At the moment there are two search modes:
  -m for minimal which scans the sequence and returns the amount of times that the pattern occurs
  -v for verbose which returns each instance that the pattern occurs along with the flanking sequences which at the moment is 5 nucleotides long

Future direction
  - create an prompt that asks how many nucleotides in the flanking sequence you want returned
  - a written output so that the search reults can be written into a new file

New ideas are welcome and appreciated
