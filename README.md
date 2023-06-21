# timecalc

Calculator for converting between units of time as well as some simple arithmetic operations

## Capabilities

### Arithmetic operations

- addition
- subtraction

### Units

- seconds
- milliseconds
- microseconds
- nanoseconds

The different mappings of text to unit can be seen in `parse_time_unit` in [src/parse.rs](src/parse.rs).

### Unit conversions

Units are left-associative so the output will be the same unit as the left-most unit.
Output can be explicitly set to a given unit by writing `as <time_unit>` at the end of an expression.

Resolution is at the nanosecond range and output is floating point. `format!()`'s default behavior for floating point
values works well enough for this use-case.

```bash
$ tcalc 1 us - 750 ns
Result: 0.25us
```

## Install locally with cargo

```bash
cargo install --path .
# For most environments, installs to ~/.cargo/bin/tcalc
```

## Example usage

Adding two different units of time

```bash
$ tcalc 2000us + 1000ns
Result: 2001us
```

Change unit of output

```bash
$ tcalc 2000ms - 150ms as us
Result: 1850000us
```
