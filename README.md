
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