#!/bin/bash

echo Compressing ...
time for i in `seq 1 5`; do
    echo Compressing $i ...; 
    tar -czvf file$i.tar file$i.txt
done;
echo Compressing done
./rust_parallel rm {1}.tar ::: file1 file2 file3 file4 file5
