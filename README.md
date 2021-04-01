# RUST PARALLEL

## Client Usage
rust_parallel [options] [command [arguments | {} | {}]] (:::+ arguments)

## Server Usage
rust_parallel -p PORT

:warning: is not yet implemented
        
## Options
    --help
    -h
        To get more information
    

    --dry-run
        Allow to display the commands without executing them

    --keep-order
        Allow to display the returns of the commands in the execution order given in input

    --jobs NB
    -j NB
        the number of threads (NB) to be used in the execution environment

    --pipe
        :warning: is not yet implemented

    --server X.X.X.X:PORT
    -s X.X.X.X:PORT
        :warning: is not yet implemented


## Example
    parallel echo ::: a b c ::: 1 2 3
    println!("\tparallel echo {0} {1}::: a b c ::: 1 2 3
