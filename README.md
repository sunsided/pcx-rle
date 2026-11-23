# pcx-rle

PCX run-length coding with four ways to get it wrong, for the article
["All tests pass"](https://markusmayer.dev/writing/all-tests-pass/) on markusmayer.dev.

The codec is correct by default. Each of its bugs is a Cargo feature:

| Feature         | Bug                                                         |
| --------------- | ----------------------------------------------------------- |
| `bug-escape`    | B1: a literal of `0xC0` or more is written bare             |
| `bug-cap`       | B2: a run of 64 is written as `0xC0 \| 64`, a count of zero |
| `bug-unchecked` | B3: `decode_fast` reads one byte past its input             |

B4 is the decoder from "Sixty-four thousand bytes", copied into
`tests/differential.rs`; its test is `#[ignore]`d and runs with `--ignored`.

```sh
cargo test                                   # unit, table, property, metamorphic, differential
cargo test --features bug-cap --test runs    # one bug, one test
cargo +nightly miri test --features bug-unchecked --test fast
cargo +nightly fuzz run --features bug-cap round_trip
cargo kani --exact --harness proofs::round_trip_4
cargo kani --features bug-escape --exact --harness proofs::round_trip_4 \
  -Z concrete-playback --concrete-playback=print
cargo mutants -- --lib                       # unit tests only
```

With [Task](https://taskfile.dev), `task bug:escape`, `task bug:cap`,
`task bug:unchecked` and `task bug:article` run the suite against one bug
each; extra arguments go after `--`, as in `task bug:cap -- --test runs`.

Proptest under Miri needs `MIRIFLAGS=-Zmiri-disable-isolation`, since it
looks for its regressions file.

## License

To the extent possible under law, the author has waived all copyright and
related or neighboring rights to this example. See [LICENSE](LICENSE) for the
CC0 1.0 Universal dedication.
