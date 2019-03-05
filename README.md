Demonstrate bug in criterion with name truncation
===

Given 2 benchmarks with very long names that start with the same
sequence of letters, the first benchmark is taken as a baseline for the
second one.

To reproduce the bug, run `cargo bench`

```
a_bench_with_an_obnoxiously_ridiculously_amazingly_stupidly_long_name_fast
                        time:   [1.4338 ns 1.4368 ns 1.4402 ns]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  5 (5.00%) high severe

a_bench_with_an_obnoxiously_ridiculously_amazingly_stupidly_long_name_slow
                        time:   [1.1065 ms 1.1093 ms 1.1123 ms]
                        change: [+76366352% +76777977% +77145093%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) low mild
```

Observe that the name was truncated at 65 characters in the report
folder, leading to the confusion:
```
ls target/criterion
a_bench_with_an_obnoxiously_ridiculously_amazingly_stupidly_long  report
```
