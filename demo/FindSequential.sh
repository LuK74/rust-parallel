#!/bin/bash

echo Looking for the file parallel.rs
time for i in `seq 1 3`; do
    if [ $i = 1 ] 
    then
        echo Looking for the file parallel.rs in the /home/ directory;
        find /home -name "parallel.rs"
    elif [ $i = 2 ] 
    then
        echo Looking for the file parallel.rs in the /etc/ directory;
        find /etc -name "parallel.rs"
    else
        echo Looking for the file parallel.rs in the /tmp/ directory;
        find /tmp -name "parallel.rs"
    fi
done;
echo Done looking
