#!/bin/bash 

echo "Brute forcing ..."
time for i in `seq 1111 6666`; do ./PasswordCrack3.sh $i; done
echo "Brute forcing done"
