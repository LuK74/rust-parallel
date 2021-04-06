#!/bin/bash

echo Looking for the file mod.rs
time for i in `seq 1 3`; do
    if [ $i = 1 ] 
    then
        echo Looking for the file parallel.rs in the /home/ directory;
        find /home -name "mod.rs"
    elif [ $i = 2 ] 
    then
        echo Looking for the file parallel.rs in the /etc/ directory;
        find /etc -name "mod.rs"
    else
        echo Looking for the file parallel.rs in the /tmp/ directory;
        find /tmp -name "mod.rs"
    fi
done;
echo Done looking
