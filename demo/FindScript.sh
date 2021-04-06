#!/bin/bash

echo Looking for the file parallel.rs
time ./rust_parallel "echo Looking for the file parallel.rs in the {1}/ directory; find {1} -name "parallel.rs" " ::: /home /etc /tmp
echo Done looking
