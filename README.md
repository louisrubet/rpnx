# rpnx [![License: LGPLv3](https://www.gnu.org/graphics/lgplv3-88x31.png)](https://www.gnu.org/licenses/lgpl-3.0.en.html)

A CLI RPN rust calculator with arbitrary‑precision arithmetic.

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
- **Number bases** 2-62 (bin, hex, etc.)
- **Complex numbers** with full arithmetic
- **Programs** as first-class objects
- **Control flow**: if/then/else, for/next, while/repeat, do/until
- **Variables** with three-level scoping

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
rpnx "2 10 pow"          # 1024
rpnx "0xFF 1 +"          # 0x100
rpnx "(1,2) (3,4) +"     # (4,6)
```

## Operations

| Category   | Commands                                                                                                                                              |
| ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| Arithmetic | `+` `-` `*` `/` `^` `pow` `neg` `chs` `inv` `sqrt` `sq` `abs` `mod` `%` `%inv` `%CH` `fact` `mant` `xpon` `floor` `ceil` `ip` `fp` `min` `max` `sign` |
| Bitwise    | `&` `\|` `^` `~`                                                                                                                                      |
| Stack      | `swap` `dup` `drop` `pop` `rot` `roll` `rolld` `pick` `depth` `over` `dup2` `dupn` `drop2` `dropn` `erase` `del` `clear` `edit`                       |
| Compare    | `>` `>=` `<` `<=` `==` `!=` `and` `or` `xor` `not` `same`                                                                                             |
| Trig       | `sin` `cos` `tan` `asin` `acos` `atan` `atan2` `d->r` `r->d` `pi`                                                                                     |
| Hyperbolic | `sinh` `cosh` `tanh` `asinh` `acosh` `atanh`                                                                                                          |
| Logs       | `ln` `log` `lnp1` `exp` `expm` `log10` `alog10` `exp10` `log2` `alog2` `exp2` `logn` `alogn` `e`                                                      |
| Complex    | `re` `im` `arg` `conj` `c->r` `r->c` `p->r` `r->p`                                                                                                    |
| String     | `->str` `str->` `chr` `num` `size` `pos` `sub` `endl`                                                                                                 |
| Variables  | `sto` `rcl` `purge` `sto+` `sto-` `sto*` `sto/` `sneg` `stoneg` `sinv` `stoinv` `vars` `clusr`                                                        |
| Control    | `if` `then` `else` `end` `ift` `ifte` `for` `next` `step` `while` `repeat` `do` `until` `start`                                                       |
| Display    | `std` `fix` `sci` `prec` `hex` `dec` `bin` `base`                                                                                                     |
| Misc       | `eval` `test` `type` `default` `error` `strerror` `date` `time` `ticks` `history` `help` `h` `?` `quit` `q` `exit` `version` `uname`                  |

## Examples

### Variables & Programs

```
rpnx> 42 'answer' sto
rpnx> answer 1 +
1> 43

rpnx> << dup * >> 'sq' sto
rpnx> 7 sq
1> 49
```

### Number Bases

```
rpnx> 0b1010 0b0110 +
1> 0b10000

rpnx> 16bff dec
1> 255

rpnx> 1000 36 base
1> 36brs
```

### Complex Numbers

```
rpnx> (3,4) abs
1> 5

rpnx> (1,0) (0,1) *
1> (0,1)
```

### More examples

[test] directory contains human-readable markdown files, functionaly validating `rpnx`. They are full of examples.

## License

LGPL-3.0 — See [LICENSE](LICENSE) file.
