#!/bin/bash

rm file1.txt file2.txt file3.txt file4.txt file5.txt
for i in `seq 1 5`; do
    touch file$i.txt; 
    for j in `seq 1 10000000`; do 
        echo $j >> file$i.txt
    done;
done
echo Generation done
