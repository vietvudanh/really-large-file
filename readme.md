# Process really large file

Inspired by this.

## Tasks

- Write a program that will print out the total number of lines in the file.
- Notice that the 8th column contains a person’s name. Write a program that loads in this data and creates an array with all name strings. Print out the 432nd and 43243rd names.
- Notice that the 5th column contains a form of date. Count how many donations occurred in each month and print out the results.
- Notice that the 8th column contains a person’s name. Create an array with each first name. Identify the most common first name in the data and how many times it occurs.

## Getting the data file

`run.sh` expects the data file at `~/data/misc/itcont.txt`. It is not checked into this repo
(see `.gitignore`).

Download it from the FEC bulk data page:

    https://www.fec.gov/files/bulk-downloads/2018/indiv18.zip

(that URL redirects to an S3 bucket, e.g.
`https://cg-519a459a-0ea3-42c2-b7bc-fa1143481f74.s3-us-gov-west-1.amazonaws.com/bulk-downloads/2018/indiv18.zip`
— the bucket name may change over time, so prefer the `fec.gov` link above).

As of writing the zip is ~1.6GB and unzips to an `itcont.txt` several GB in size. After
downloading, unzip it and move it into place:

    mkdir -p ~/data/misc
    unzip indiv18.zip -d ~/data/misc

# Lesson learnt

    - pypy is fucking slow, what?
    - solution at https://itnext.io/using-java-to-read-really-really-large-files-a6f8a3f44649 is fucking bad, GC overhead all the time