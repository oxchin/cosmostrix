# Benchmark

This folder contains performance and profiling artifacts for Cosmostrix.

## Summary (current results)

- Peak heap (Valgrind Massif): `~212 KB` (`212,904 bytes` total)
- Heap behavior: stable (no obvious growth during runtime)
- Release build is significantly faster than debug (use `hyperfine` for exact numbers on your machine)

Practical takeaway:

- Memory usage is low and stable for a terminal visualizer.
- If you want a stronger claim ("no leaks"), run a longer Massif capture or use a leak checker tool.

Massif detail (from `massif.out` peak snapshot):

- `mem_heap_B=212316`
- `mem_heap_extra_B=588`
- Total: `212316 + 588 = 212904 bytes`

Artifacts currently tracked here:

- `massif.out`: Massif output (heap snapshots)
- `flamegraph.svg`: CPU flamegraph (SVG)

Large/temporary artifacts:

- `perf.data`: raw perf recording (large)

Recommendation: don’t commit `perf.data` long-term (it is big and highly machine-specific). This folder has a local `.gitignore` to ignore `perf.data*` for future recordings.

If `perf.data` is already tracked in git, ignoring it is not enough. Untrack it with:

```bash
git rm --cached benchmark/perf.data
git commit -m "chore(bench): stop tracking perf.data"
```

## Reproducible 30s benchmark

Recommended approach for consistent benchmarking is to run Cosmostrix with a fixed duration so it can exit cleanly:

```bash
cargo build
cargo build --release

./target/release/cosmostrix --duration 30
```

If you want to compare debug vs release manually:

```bash
./target/debug/cosmostrix --duration 30
./target/release/cosmostrix --duration 30
```

Then run the benchmark script:

```bash
bash benchmark/benchmark.sh
```

The script will try to generate:

- `benchmark/hyperfine.md`
- `benchmark/time-release.txt`, `benchmark/time-debug.txt`
- `benchmark/perf-release.txt`, `benchmark/perf-debug.txt`
- `benchmark/massif-30s.out`

(If a tool is missing, the related step is skipped.)
