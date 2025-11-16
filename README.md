# Profile

```bash
pgrep -af wordle-solver # get process id
#12345
```

## Linux

```bash
perf record --call-graph dwarf -p 12345
perf script | inferno-collapse-perf | inferno-flamegraph > perf.svg
```

## MacOS

```bash
sudo dtrace -x ustackframes=100 \
  -n 'profile-997 /pid == $target/ { @[ustack()] = count(); }' \
  -o out.stacks -p (pgrep -af wordle-solver)
FlameGraph/stackcollapse.pl out.stacks > out.folded
FlameGraph/flamegraph.pl out.folded > out.svg
```

# Reference

- [Implementing and Optimizing a Wordle Solver in Rust](https://youtu.be/doFowk4xj7Q)
- [jonhoo/roget](https://github.com/jonhoo/roget)
