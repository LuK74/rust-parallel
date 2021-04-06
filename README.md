# RUST PARALLEL

## Client Usage
rust_parallel [options] [command [arguments | {[n]}]] ::: values

## Server Usage
rust_parallel --server PORT
        
## Options list
+ --help                  display this message
+ --dry-run               display the jobs without executing them
+ --server PORT           launch as a remote executor machine listening on PORT
+ --client IP_DST PORT    launch all the jobs remotly on machine IP_DST:PORT
+ --keep-order            allow to display the returns of the commands in the execution order given in input
+ --jobs NB / -j NB       the number of threads (NB) to be used (0 = unlimited)
+ --pipe                  is not yet implemented


## Example
+ parallel echo ::: a b c ::: 1 2 3
+ parallel echo {2} {1}::: a b c ::: 1 2 3Z
