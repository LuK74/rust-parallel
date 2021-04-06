#!/bin/bash

echo Compressing ...
time ./rust_parallel "echo Compressing {1} ...; tar -czvf {1}.tar {1}.txt" ::: file1 file2 file3 file4 file5
echo Compressing done
./rust_parallel rm {1}.tar ::: file1 file2 file3 file4 file5
