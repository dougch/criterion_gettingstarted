
### Cargo bench

Runs like any other rust criterion benchmark:
```
cargo bench
```
### FlameGraph

```
cargo install flamegraph
cargo flamegraph --bench flamegraph -- --profile-time=5
```

Note, this uses pperf on linux, or dtrace on other platforms and may require
 root priviledges not available on all CIs/platforms.


 ### Github Action

 The Github action has issues with the default criterion output, so the extra options are
 needed to correctly parse the output.  Note the benchmark names should not contain spaces or dashes (regis pattern is \w).