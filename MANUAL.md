# **rpnx v1.0.0** - Reference Manual

- structured full-featured **math-oriented language** inspired by Hewlett-Packard **R**everse **P**olish **L**isp, including **real**, **bitwise**, **complex**, **stack**, **store**, **branch**, **program**, **test**, **trig** and **logs** commands,
- implemented in rust as a **command-line calculator** with interactive REPL,
- bringing powerfull calculation facilities on floating point numbers with __arbitrary precision__ provided by [rust crate](https://crates.io/crates/rug),
- uses the **reverse polish notation**.

## Doc overview

This page exhaustively lists the currently implemented commands and gives some examples.

A help command is provided by `rpnx`:

```rpnx
rpnx> help

rpnx v1.0.0 - Reverse Polish Notation Calculator

(...)
GENERAL
help    this help message
(...)
```

Command-specific help is available with the syntax `'command' help`:

```rpnx
rpnx> 'sto' help

sto: Store a value in a variable
syntax: value 'name' sto
  value: any object to store
  name: quoted symbol, variable name
example: 42 'answer' sto
```

In interactive mode only, a shorter syntax is available: `help command` (or `h command` or `? command`):

```rpnx
rpnx> help +

+: Add two numbers or concatenate symbols
syntax: a b +
  a: number, complex, or symbol
  b: number, complex, or symbol
example: 1 2 +
```

## Quick examples

### Calculation with **stacked results**

```rpnx
rpnx> 1 2 +
3
rpnx> 2 sqrt
2> 3
1> 1.4142135623730950488016887242096980786
```

### **Bitwise operations**, regardless of the number of bits or the representation

```rpnx
rpnx> 0b110111 0b101011 &
0b100011
```

```rpnx
rpnx> 0x37 0b101011 ^
0x1c
```

### **Programs** and **variables**

```rpnx
rpnx> << rot * swap 2 / neg dup sq rot - sqrt >> 'quadratic_solution' sto
rpnx> 1 2 -3 quadratic_solution
2> -1
1> 2
rpnx> vars
Global variables:
  var 1: name 'quadratic_solution', type program
```

### **Local variables**

```rpnx
rpnx> << -> x y << x y + ln >> >> 'P' sto
rpnx> 1 2 P
rpnx> 1.0986122886681096913952452369225257046
```

### **Arbitrary precision**

The number of significant digits can be up to 1000000

```rpnx
rpnx> 256 prec
rpnx> pi
3.1415926535897932384626433832795028841971693993751058209749445923078164062862
rpnx> del 10000 prec pi
3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117067982148086513282306647093844609550582231725359408128481117450284102701938521105559644622948954930381964428810975665933446128475648233786783165271201909145648566923460348610454326648213393607260249141273724587006606315588174881520920962829254091715364367892590360011330530548820466521384146951941511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949129833673362440656643086021394946395224737190702179860943702770539217176293176752384674818467669405132000568127145263560827785771342757789609173637178721468440901224953430146549585371050792279689258923542019956112129021960864034418159813629774771309960518707211349999998372978049951059731732816096318595024459455346908302642522308253344685035261931188171010003137838752886587533208381420617177669147303598253490428755468731159562863882353787593751957781857780532171226806613001927876611195909216420198938095257201065485863278865936153381827968230301952035301852968995773622599413891249721775283479131515574857242454150695950829533116861727855889075098381754637464939319255060400927701671139009848824012858361603563707660104710181942955596198946767837449448255379774726847104047534646208046684259069491293313677028989152104752162056966024058038150193511253382430035587640247496473263914199272604269922796782354781636009341721641219924586315030286182974555706749838505494588586926995690927210797509302955321165344987202755960236480665499119881834797753566369807426542527862551818417574672890977772793800081647060016145249192173217214772350141441973568548161361157352552133475741849468438523323907394143334547762416862518983569485562099219222184272550254256887671790494601653466804988627232791786085784383827967976681454100953883786360950680064225125205117392984896084128488626945604241965285022210661186306744278622039194945047123713786960956364371917287467764657573962413890865832645995813390478027590099465764078951269468398352595709825822620522489407726719478268482601476990902640136394437455305068203496252451749399651431429809190659250937221696461515709858387410597885959772975498930161753928468138268683868942774155991855925245953959431049972524680845987273644695848653836736222626099124608051243884390451244136549762780797715691435997700129616089441694868555848406353422072225828488648158456028506016842739452267467678895252138522549954666727823986456596116354886230577456498035593634568174324112515076069479451096596094025228879710893145669136867228748940560101503308617928680920874760917824938589009714909675985261365549781893129784821682998948722658804857564014270477555132379641451523746234364542858444795265867821051141354735739523113427166102135969536231442952484937187110145765403590279934403742007310578539062198387447808478489683321445713868751943506430218453191048481005370614680674919278191197939952061419663428754440643745123718192179998391015919561814675142691239748940907186494231961567945208
rpnx> 
```

### Object types

The following objects are managed: **floating numbers**, **complexes**, **symbols**, **programs**, plus language **keywords** (commands and flow controls).

```rpnx
4> 12.3456
3> (1,-2.33)
2> 'symbol'
1> << -> n << 0 1 n for i i 2 * inv + next >> >>
rpnx> 
```

### Numbers representation

- **reals** can be entered in decimal, binary, hexadecimal or arbitrary base from 2 to 62.
  - Binaries (base 2) are entered as `0b<number>` or `0B<number>`, ex: `0b1110110`
  - Hexadecimals (base 16) are entered as `0x<number>` or `0X<number>`, ex: `0xcafe`
  - Arbitrary base numbers are entered as `<base>b<number>`, ex: `5b14330`
  - Representation can be individually changed with keywords `dec`, `bin`, `hex` and `<n> base`.

Ex:

```rpnx
rpnx> 5.6 0xaabb 0b1101 7b1252
4> 5.6
3> 0xaabb
2> 0b1101
1> 7b1252

rpnx> 7b1252 dec
1> 478
```

- **Decimal reals** can be represented in 3 ways:
  - **std**: Standard display shows up to n significant digits without trailing zeros.
    - Syntax: `<n> std`
    - Example: `38 std 1000 3 /` → `333.33333333333333333333333333333333333`
  - **fix**: Fixed-point display shows exactly n digits after the decimal point.
    - Syntax: `<n> fix`
    - Example: `4 fix 1000 3 /` → `333.3333`
  - **sci**: Scientific notation displays n significant digits with exponent.
    - Syntax: `<n> sci`
    - Example: `6 sci 1000 3 /` → `3.333333e+02`

The whole stack is represented with the chosen mode.

### Sticky operators

Arithmetic and bitwise operators `+ - * / % & ^ ~` can stick at the immediate right of a number for convenience.

```rpnx
rpnx> 1 2+
3
```

### structured programming

Loop keywords **for|start..next|step**, **do..until**, **while..repeat** and control keywords **if..then..[else]..end**, **ift**, **ifte** allow you to program powerfull algorithms.

### command line

**rpnx** is a cli interface with REPL (**interactive editor** with autocompletion).

Autocompletion works like those in Linux shells, with keys \<tab\>, Ctrl-R \<search pattern\> etc.

## keywords

### general

| keyword           | description                                                      |
| ----------------- | ---------------------------------------------------------------- |
| `help` `h` `?`    | this help message, or `'command' help` for command-specific help |
| `quit` `q` `exit` | quit software                                                    |
| `version`         | show rpnx version                                                |
| `uname`           | show rpnx complete identification string                         |
| `history`         | see commands history                                             |
| `error`           | last encountered error                                           |
| `strerror`        | last encountered error string                                    |
| `test`            | run a test file                                                  |

### usual operations - real and complex

| keyword     | description                                                      |
| ----------- | ---------------------------------------------------------------- |
| `+`         | addition                                                         |
| `-`         | substraction                                                     |
| `neg` `chs` | negation                                                         |
| `*`         | multiplication                                                   |
| `/`         | division                                                         |
| `inv`       | inverse                                                          |
| `pow`       | power                                                            |
| `sqrt`      | square root                                                      |
| `sq` `sqr`  | square                                                           |
| `abs`       | absolute value for a number or `sqrt(re*re+im*im)` for a complex |
| `dec`       | decimal representation                                           |
| `hex`       | hexadecimal representation                                       |
| `bin`       | binary representation                                            |
| `base`      | arbitrary base representation                                    |
| `sign`      | sign of a real, unary vector in the same direction for a complex |

### bitwise operations

| operator | description              |
| -------- | ------------------------ |
| `&`      | bitwise and              |
| `\|`     | bitwise or               |
| `^`      | bitwise xor              |
| `~`      | bitwise not (complement) |

### operations on reals

| keyword | description                                     |
| ------- | ----------------------------------------------- |
| `%`     | purcent                                         |
| `%inv`  | inverse purcent                                 |
| `mod`   | modulo                                          |
| `fact`  | n! for integer n or Gamma(x+1) for fractional x |
| `mant`  | mantissa of a real number                       |
| `xpon`  | exponant of a real number                       |
| `floor` | largest number <=                               |
| `ceil`  | smallest number >=                              |
| `ip`    | integer part                                    |
| `fp`    | fractional part                                 |
| `min`   | min of 2 real numbers                           |
| `max`   | max of 2 real numbers                           |

### operations on complexes

| keyword | description                      |
| ------- | -------------------------------- |
| `re`    | complex real part                |
| `im`    | complex imaginary part           |
| `conj`  | complex conjugate                |
| `arg`   | complex argument in radians      |
| `r->p`  | rectangular to polar coordinates |
| `p->r`  | polar to rectangular coordinates |
| `r->c`  | transform 2 reals in a complex   |
| `c->r`  | transform a complex in 2 reals   |

### mode

| keyword   | description                                                  |
| --------- | ------------------------------------------------------------ |
| `std`     | standard floating numbers representation. ex: `std`          |
| `fix`     | fixed point representation. ex: `6 fix`                      |
| `sci`     | scientific floating point representation. ex: `20 sci`       |
| `prec`    | set float precision in bits from 2 to 100000. ex: `256 prec` |
| `default` | set float representation and precision to default            |
| `type`    | show type of stack first entry                               |

`std`, `fix`, `sci` are display-mode only, and act on all stack levels.

`prec` acts on the numbers entered subsequently.

### test

| keyword | description                    |
| ------- | ------------------------------ |
| `>`     | binary operator >              |
| `>=`    | binary operator >=             |
| `<`     | binary operator <              |
| `<=`    | binary operator <=             |
| `!=`    | binary operator != (different) |
| `==`    | binary operator == (equal)     |
| `and`   | boolean operator and           |
| `or`    | boolean operator or            |
| `xor`   | boolean operator xor           |
| `not`   | boolean operator not           |
| `same`  | boolean operator same (equal)  |

### stack

| keyword               | description                                                     |
| --------------------- | --------------------------------------------------------------- |
| `swap`                | swap 2 first stack entries                                      |
| `drop` `pop`          | drop first stack entry                                          |
| `drop2`               | drop 2 first stack entries                                      |
| `dropn`               | drop n first stack entries                                      |
| `del` `erase` `clear` | drop all stack entries                                          |
| `rot`                 | rotate 3 first stack entries                                    |
| `dup`                 | duplicate first stack entry                                     |
| `dup2`                | duplicate 2 first stack entries                                 |
| `dupn`                | duplicate n first stack entries                                 |
| `pick`                | push a copy of  the given stack level onto the stack            |
| `depth`               | give stack depth                                                |
| `roll`                | move a stack entry to the top of the stack                      |
| `rolld`               | move the element on top of the stack to a higher stack position |
| `over`                | push a copy of the element in stack level 2 onto the stack      |

### string

| keyword | description                                                                              |
| ------- | ---------------------------------------------------------------------------------------- |
| `->str` | convert an object into a string                                                          |
| `str->` | convert a string into an object                                                          |
| `chr`   | convert ASCII character code in stack level 1 into a string                              |
| `num`   | return ASCII code of the first character of the string in stack level 1 as a real number |
| `size`  | return the length of the string                                                          |
| `pos`   | seach for the string in level 1 within the string in level 2                             |
| `sub`   | return a substring of the string in level 3                                              |
| `endl`  | end-of-line string                                                                       |

### branch

| keyword  | description                                                                                           |
| -------- | ----------------------------------------------------------------------------------------------------- |
| `if`     | `(test-instruction) if then (true-instructions) else (false-instructions) end`                        |
| `then`   | used with if                                                                                          |
| `else`   | used with if                                                                                          |
| `end`    | used with various branch instructions                                                                 |
| `ift`    | similar to if-then-end: `(test-instruction) (true-instruction) ift`                                   |
| `ifte`   | similar to if-then-else-end: `(test-instruction) (true-instruction) (false-instruction) ifte`         |
| `start`  | `(start) (end) start (instructions) [next,(step) step]`. ex: `10 20 30 1 2 start + next`              |
| `for`    | `(start) (end) for (variable) (instructions) [next,(step) step]`. ex: `1 2 for i i 'a' sto+ 0.1 step` |
| `next`   | used with start and for                                                                               |
| `step`   | used with start and for                                                                               |
| `do`     | `do (instructions) until (condition) end`                                                             |
| `until`  | used with do                                                                                          |
| `while`  | `while (test-instruction) repeat (loop-instructions) end`                                             |
| `repeat` | used with while                                                                                       |

### store

| keyword         | description                                                     |
| --------------- | --------------------------------------------------------------- |
| `sto`           | store a variable. ex: ```1 'name' sto```                        |
| `rcl`           | recall a variable. ex: ```'name' rcl```                         |
| `purge`         | delete a variable. ex: ```'name' purge```                       |
| `vars`          | list all variables                                              |
| `clusr`         | erase all variables                                             |
| `edit`          | edit a variable content                                         |
| `sto+`          | add to a stored variable. ex: 1 'name' sto+ 'name' 2 sto+       |
| `sto-`          | substract to a stored variable. ex: 1 'name' sto- 'name' 2 sto- |
| `sto*`          | multiply a stored variable. ex: 3 'name' sto* 'name' 2 sto*     |
| `sto/`          | divide a stored variable. ex: 3 'name' sto/ 'name' 2 sto/       |
| `stoneg` `sneg` | negate a variable. ex: 'name' sneg                              |
| `stoinv` `sinv` | inverse a variable. ex: 1 'name' sinv                           |

### program

| keyword | description                                                                 |
| ------- | --------------------------------------------------------------------------- |
| `eval`  | evaluate (run) a program, or recall a variable. ex: `'my_prog' eval`        |
| `->`    | load program local variables. ex: `<< -> n m << 0 n m for i i + next >> >>` |

### trig on reals and complexes

| keyword | description                |
| ------- | -------------------------- |
| `pi`    | pi constant                |
| `sin`   | sine                       |
| `asin`  | arcsine                    |
| `cos`   | cosine                     |
| `acos`  | arccosine                  |
| `tan`   | tangent                    |
| `atan`  | arctangent                 |
| `atan`  | two-argument arctangent    |
| `d->r`  | convert degrees to radians |
| `r->d`  | convert radians to degrees |

### logs on reals and complexes

| keyword          | description                                   |
| ---------------- | --------------------------------------------- |
| `e`              | Euler constant                                |
| `ln` `log`       | logarithm base e                              |
| `lnp1`           | ln(1+x) which is useful when x is close to 0  |
| `exp`            | exponential                                   |
| `expm`           | exp(x)-1 which is useful when x is close to 0 |
| `log10`          | logarithm base 10                             |
| `alog10` `exp10` | exponential base 10                           |
| `log2`           | logarithm base 2                              |
| `alog2` `exp2`   | exponential base 2                            |
| `logn`           | logarithm base n                              |
| `sinh`           | hyperbolic sine                               |
| `asinh`          | inverse hyperbolic sine                       |
| `cosh`           | hyperbolic cosine                             |
| `acosh`          | inverse hyperbolic cosine                     |
| `tanh`           | hyperbolic tangent                            |
| `atanh`          | inverse hyperbolic tangent                    |
| `time`           | local time in ISO 8601 format HH:MM:SS        |
| `date`           | local date in ISO 8601 format YYYY-MM-DD      |
| `ticks`          | local date and time in µs                     |

### default

Default float mode is 'std' with 38 digits

Default floating point precision is 128 bits

## Tests

- A set of complete test sheets are given in the [test](https://github.com/louisrubet/rpnx/tree/master/test) subdirectory.

- Runnable by `test` command, ex: `cd test && rpnx all.md test`
