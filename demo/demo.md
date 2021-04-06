# Examples

# Performances 

## Brute Force password crack

### Parralel command
>./rust_parallel --jobs 6 bash PasswordCrack.sh {1}{2}{3}{4}{5}{6} ::: 1 2 3 4 5 6 ::: 1 2 3 4 5 6 ::: 1 2 3 4 5 6 ::: 1 2 3 4 5 6 ::: 1 2 3 4 5 6 ::: 1 2 3 4 5 6

### Sequential command
for i in `seq 111111 666666`; do ./PasswordCrack.sh $i; done

### Time
Parralel : 
real	0m19.378s
user	1m29.414s
sys	    1m6.337s

Sequential : 
real	28m56.836s
user	10m51.210s
sys	    18m34.264s

## Brute Force password crack 2

### Parralel command
>./rust_parallel --jobs 6 bash PasswordCrack2.sh {1}{2}{3}{4}{5} ::: 1 2 3 4 5 ::: 1 2 3 4 5 ::: 1 2 3 4 5 ::: 1 2 3 4 5 ::: 1 2 3 4 5

### Sequential command
>for i in `seq 11111 66666`; do ./PasswordCrack2.sh $i; done

### Time
Parallel :
real	0m1.044s
user	0m5.461s
sys	    0m3.385s

Sequential : 
real	1m8.464s
user	0m49.925s
sys	    0m21.114s