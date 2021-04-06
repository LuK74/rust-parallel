#!/bin/bash 

echo "Brute forcing ..."
time ./rust_parallel --jobs 6 "bash PasswordCrack3.sh {1}{2}{3}{4}" ::: 1 2 3 4 ::: 1 2 3 4 ::: 1 2 3 4 ::: 1 2 3 4
echo "Brute forcing done"
