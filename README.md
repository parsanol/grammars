# parsanol/grammars

The general-purpose grammar registry: **data-format flavors** live
here, with the full gate stack. Publication-identifier flavors live
in [pubid/pubid-grammar](https://github.com/pubid/pubid-grammar) —
this repo exists because data formats do not belong there
(TODO.sota/10, owner directive 2026-11-16).

## Flavors

| grammar | spec | notes |
|---|---|---|
| `json` | RFC 8259 | the shared corpus grammar for the industry benchmark suite (parsanol-rs#526) |
| `csv` | RFC 4180 shape | |
| `ini` | the common INI shape | |
| `toml` | the configuration-language grammar (rs#531 breadth) | |
| `lua` | the lua-subset: statement-oriented syntax (rs#531 breadth) | |

## Layout

- `grammars/*.parg` — the grammar sources (PARG, edition v2).
- `artifacts/*.json` — the sealed artifacts, baked with the released
  compiler pinned in CI (`PARSANOL_VERSION`).
- `corpora/*/corpus.json` — the frozen corpora: every case records
  the artifact checksum and the native engine's tree/bound output.

## Gates (CI)

- **grammar**: every grammar compiles and passes its tests
  (`parsanol parg test`), and every corpus's recorded checksum
  matches its sealed artifact.
- **rust**: the artifacts replay through the native engine
  (parsanol-rs main, the bindings suite).
- **typescript**: the artifacts replay through the wasm engine
  (parsanol-ts main).

A grammar change must carry its re-baked corpora and re-sealed
artifacts — the freeze check fails any drift.
