#!/bin/bash

./rust_parallel --keep-order echo {1} is before {2} ::: 1 2 3 ::: 4 5 6
