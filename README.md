# timecalc

Calculator for converting between units of time as well as some simple arithmetic operations

## Capabilities

It's a pretty dead-simple calculator for simple conversions.

Output can be explicitly set to a given unit by writing `as <time_unit>` at the end of an expression.

Changing a larger unit like `seconds` by a very small amount will be lossy since only integer outputs are used.

By default, rust rounds down in integer conversion so doing `90sec - 1ns` results in `89sec` if the output format is in `seconds`.
Using the conversion operator `as ns` will prevent the output from rounding down and `89999999999ns` will be the output instead.

### arithmetic operations

- addition
- subtraction

### units

- seconds
- milliseconds
- microseconds
- nanoseconds

The different mappings of text to unit can be seen in `parse_time_unit` in [src/parse.rs](src/parse.rs).

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
