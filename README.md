# datafusion_malloc_bench

This little project aims to look at differences between memory allocators on real world task. 
For this purpose I've picked `Arrow DataFusion`, modern query execution framework.

## Benchmarked allocators

- default
- jemalloc
- mimalloc
- snmalloc
