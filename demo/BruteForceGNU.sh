#!/bin/bash 

echo "Brute forcing ..."
time parallel --jobs 6 "bash PasswordCrack.sh {1}{2}{3}{4}{5}{6} " ::: 1 2 3 4 5 6 ::: 1 2 3 4 5 6 ::: 1 2 3 4 5 6 ::: 1 2 3 4 5 6 ::: 1 2 3 4 5 6 ::: 1 2 3 4 5 6
echo "Brute forcing done"
