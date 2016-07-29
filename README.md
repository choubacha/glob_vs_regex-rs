Benchmarks performance differences for a simple regex and glob
that should basically do the same thing

Here's the results in debug:

```
$ target/debug/glob_vs_regex
a bunch of regex
took 4.946 s total
aver 0.004946 s/iter
a bunch of globs
took 6.435 s total
aver 0.006435 s/iter
```

And compiled as release:

```
$ target/release/glob_vs_regex
a bunch of regex
took 0.149 s total
aver 0.000149 s/iter
a bunch of globs
took 0.334 s total
aver 0.000334 s/iter
```
