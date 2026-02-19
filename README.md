 rpnx [![License: LGPLv3](https://www.gnu.org/graphics/lgplv3-88x31.png)](https://www.gnu.org/licenses/lgpl-3.0.en.html)

A CLI RPN rust calculator language with arbitrary‑precision arithmetic.

## Install from source

```bash
git clone https://github.com/louisrubet/rpnx
cd rpnx
cargo build --release
cargo install --path .
```

## Features

- **Arbitrary precision**
- **Full arithmetic and scientific functions**
- **Bitwise operations**
- **Number bases** dec, bin, hex, bases 3 to 62
- **Complex numbers** with full arithmetic
- **Programs** with parameters
- **Control flow** if/then/else, for/next, while/repeat, do/until
- **Variables** storing 

Complete manual in [MANUAL.md](MANUAL.md)

## Quick Start

Interactive

```bash
$ rpnx
rpnx> 2 3 + dup *
25

rpnx> sqrt
5

rpnx> pi 2 / sin
2> 5
1> 1

rpnx> 255 hex
3> 5
2> 1
1> 0xff

rpnx> del
rpnx> 0b110111 0b101011 &
0b100011

```

From command line

```bash
rpnx> "2 10 pow"
rpnx> "0xFF 1 +"
rpnx> "(1,2) (3,4) +"
```

## Operations

| Category   | Commands                                                                                                                                          |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| Arithmetic | `+` `-` `*` `/` `pow` `neg` `chs` `inv` `sqrt` `sq` `abs` `mod` `%` `%inv` `%CH` `fact` `mant` `xpon` `floor` `ceil` `ip` `fp` `min` `max` `sign` |
| Bitwise    | `&` `\|` `^` `~`                                                                                                                                  |
| Stack      | `swap` `dup` `drop` `pop` `rot` `roll` `rolld` `pick` `depth` `over` `dup2` `dupn` `drop2` `dropn` `erase` `del` `clear` `edit`                   |
| Compare    | `>` `>=` `<` `<=` `==` `!=` `and` `or` `xor` `not` `same`                                                                                         |
| Trig       | `sin` `cos` `tan` `asin` `acos` `atan` `atan2` `d->r` `r->d` `pi`                                                                                 |
| Hyperbolic | `sinh` `cosh` `tanh` `asinh` `acosh` `atanh`                                                                                                      |
| Logs       | `ln` `log` `lnp1` `exp` `expm` `log10` `alog10` `exp10` `log2` `alog2` `exp2` `logn` `alogn` `e`                                                  |
| Complex    | `re` `im` `arg` `conj` `c->r` `r->c` `p->r` `r->p`                                                                                                |
| Variables  | `sto` `rcl` `purge` `sto+` `sto-` `sto*` `sto/` `sneg` `stoneg` `sinv` `stoinv` `vars` `clusr`                                                    |
| Control    | `if` `then` `else` `end` `ift` `ifte` `for` `next` `step` `while` `repeat` `do` `until` `start`                                                   |
| Display    | `std` `fix` `sci` `prec` `hex` `dec` `bin` `base`                                                                                                 |
| Misc       | `eval` `test` `type` `default` `error` `strerror` `date` `time` `ticks` `history` `help` `h` `?` `quit` `q` `exit` `version` `uname`              |

## Examples

### Number Bases

```rpnx
rpnx> 0b1010 0b0110 +
0b10000

rpnx> 16bff dec
255

rpnx> 1000 36 base
36brs
```

### Complex Numbers

```rpnx
rpnx> (3,4) abs
5

rpnx> (1,0) (0,1) *
(0,1)
```

### Variables & Programs

Sum of squares
```rpnx
rpnx> 0 1 10 for i i sq + next
385
```

Tests and variables
```rpnx
rpnx> 42 'answer' sto
rpnx> << if 42 == then 'truth' else 'lie' end >> 'check_the_truth' sto
rpnx> answer check_the_truth
'truth'
rpnx> 2 'answer' sto+
rpnx> answer check_the_truth
'lie'
```

```rpnx
rpnx> << dup * >> 'sq' sto
rpnx> 7 sq
49
```

### More examples

[test] directory contains human-readable markdown files, functionaly validating `rpnx`. They are full of examples.

## License

LGPL-3.0 — See [LICENSE](LICENSE) file.

## Credits
- [Rust Programming Language](https://rust-lang.org/)
- [rug crate](https://crates.io/crates/rug): arbitrary-precision integers, rational, floating-point and complex numbers based on [GMP](https://gmplib.org/), [MPFR](https://www.mpfr.org/) and [MPC](https://www.multiprecision.org/mpc/).
- <a href="https://www.flaticon.com/free-icons/calculator" title="calculator icons">Calculator icons created by Freepik - Flaticon</a>
