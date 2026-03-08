# soil-test-fuzz

Shared fuzz harnesses for Soil crates.

## Targets

- `core_address_uri`
- `npos_reduce`
- `npos_phragmen_balancing`
- `npos_phragmms_balancing`
- `npos_phragmen_pjr`
- `plant_bags_list`
- `plant_election_compact`

## Running

Run a target with honggfuzz:

```bash
cargo hfuzz run core_address_uri
cargo hfuzz run npos_phragmen_balancing
cargo hfuzz run plant_bags_list
cargo hfuzz run plant_election_compact
```

`npos_phragmen_pjr` also supports a single local iteration without honggfuzz:

```bash
cargo run -p soil-test-fuzz --bin npos_phragmen_pjr -- --help
```

## Old To New Mapping

- `fuzz_address_uri` -> `core_address_uri`
- `reduce` -> `npos_reduce`
- `phragmen_balancing` -> `npos_phragmen_balancing`
- `phragmms_balancing` -> `npos_phragmms_balancing`
- `phragmen_pjr` -> `npos_phragmen_pjr`
- `bags-list` -> `plant_bags_list`
- `compact` -> `plant_election_compact`
