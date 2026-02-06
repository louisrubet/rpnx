# REAL NUMBER BASE REPRESENTATION INHERITANCE

## Binary operations: addition inherits left operand base

```
0x12 1 +
```

-> stack should be 0x13

`del`

```
1 0x12 +
```

-> stack should be 19

`del`

```
0b1010 0b11 +
```

-> stack should be 0b1101

`del`

## Binary operations: subtraction inherits left operand base

```
0x20 0x5 -
```

-> stack should be 0x1b

`del`

```
20 5 hex -
```

-> stack should be 15

`del`

```
0b1100 0b10 -
```

-> stack should be 0b1010

`del`

## Binary operations: multiplication inherits left operand base

```
0x10 0b10 *
```

-> stack should be 0x20

`del`

```
0b10 0x10 *
```

-> stack should be 0b100000

`del`

## Binary operations: division inherits left operand base

```
0x20 0x4 /
```

-> stack should be 0x8

`del`

```
32 4 hex /
```

-> stack should be 8

`del`

```
0b1000 0b10 /
```

-> stack should be 0b100

`del`

## Binary operations: power inherits left operand base

```
0x10 2 pow
```

-> stack should be 0x100

`del`

```
2 0x10 pow
```

-> stack should be 65536

`del`

```
0b10 3 pow
```

-> stack should be 0b1000

`del`

## Binary operations: modulo inherits left operand base

```
0x17 0x5 mod
```

-> stack should be 0x3

`del`

```
23 5 hex mod
```

-> stack should be 3

`del`

## Binary operations: min/max inherit left operand base

```
0x10 0x20 min
```

-> stack should be 0x10

`del`

```
16 32 hex max
```

-> stack should be 32

`del`

```
0b1010 0b101 max
```

-> stack should be 0b1010

`del`

## Unary operations: negation preserves base

```
0xF neg
```

-> stack should be -0xf

`del`

```
0b1010 neg
```

-> stack should be -0b1010

`del`

## Unary operations: inverse preserves base

```
0x4 hex inv
```

-> stack should be 0x0

`del`

```
4 hex inv
```

-> stack should be 0x0

`del`

## Unary operations: square root preserves base

```
0b100 sqrt
```

-> stack should be 0b10

`del`

```
0x10 hex sqrt
```

-> stack should be 0x4

`del`

## Unary operations: square preserves base

```
0x4 hex sq
```

-> stack should be 0x10

`del`

```
0b11 sq
```

-> stack should be 0b1001

`del`

## Unary operations: absolute value preserves base

```
0xF hex neg abs
```

-> stack should be 0xf

`del`

```
0b101 neg abs
```

-> stack should be 0b101

`del`

## Unary operations: sign preserves base

```
0x10 hex sign
```

-> stack should be 0x1

`del`

```
0b10 neg sign
```

-> stack should be -0b1

`del`

## Unary operations: factorial preserves base

```
0x5 hex fact
```

-> stack should be 0x78

`del`

```
0b11 fact
```

-> stack should be 0b110

`del`

## Unary operations: floor/ceil preserve base

```
0x10 hex floor
```

-> stack should be 0x10

`del`

```
0b101 ceil
```

-> stack should be 0b101

`del`

## Unary operations: integer/fractional part preserve base

```
0x10 hex ip
```

-> stack should be 0x10

`del`

```
0b101 fp
```

-> stack should be 0b0

`del`

## Trigonometric functions preserve base

```
0 hex cos
```

-> stack should be 0x1

`del`

```
0 hex sin
```

-> stack should be 0x0

`del`

```
1 asin
```

-> stack should 1.5707963267948966192313216916397514421

`del`

```
1 hex asin
```

-> stack should be 0x1

`del`

## Hyperbolic functions preserve base

```
0 hex sinh
```

-> stack should be 0x0

`del`

```
0 hex cosh
```

-> stack should be 0x1

`del`

## Logarithmic functions preserve base

```
1 hex ln
```

-> stack should be 0x0

`del`

```
1 hex exp
```

-> stack should be 0x2

`del`

```
1 hex log10
```

-> stack should be 0x0

`del`

```
1 hex log2
```

-> stack should be 0x0

`del`

## Binary logarithmic functions inherit left operand base

```
0x8 2 logn
```

-> stack should be 0x3

`del`

```
2 0x8 alogn
```

-> stack should be 64

`del`

## Conversion functions preserve base

```
180 hex d->r
```

-> stack should be 0x3

`del`

```
3.14159265358979 hex r->d
```

-> stack should be 0xb3

`del`

## atan2 inherits left operand base

```
0 hex 1 atan2
```

-> stack should be 0x1

`del`

```
1 0 hex atan2
```

-> stack should be 0

`del`

## Chain operations preserve base through multiple operations

```
0x10 1 + 2 *
```

-> stack should be 0x22

`del`

```
0b1000 0b10 / sqrt
```

-> stack should be 0b10

`del`

```
0x5 hex sq sqrt
```

-> stack should be 0x5

`del`

## Mixed bases in chain - leftmost wins

```
0x10 5 + 0b10 + 3 +
```

-> stack should be 0x1a

`del`

```
10 0x5 + 0b10 + 3 +
```

-> stack should be 20

`del`

```
0b1000 4 + 0x2 + 1 +
```

-> stack should be 0b1111

`del`

## Base explicit conversion still works

```
0x10 hex
```

-> stack should be 0x10

`del`

```
0x10 dec
```

-> stack should be 16

`del`

```
0x10 bin
```

-> stack should be 0b10000

`del`

```
16 hex
```

-> stack should be 0x10

`del`

## Operations with explicit base conversion

```
10 hex 5 +
```

-> stack should be 0xf

`del`

```
10 5 + hex
```

-> stack should be 0xf

`del`

```
0x10 dec 5 +
```

-> stack should be 21

`del`

## Complex expressions with base inheritance

```
0x2 0x3 * 0x4 + 0x5 /
```

-> stack should be 0x2

`del`

```
0b10 0b11 * 0b100 + 0b101 /
```

-> stack should be 0b10

`del`

## Edge case: operations with zero preserve left base

```
0x10 0 +
```

-> stack should be 0x10

`del`

```
0 0x10 +
```

-> stack should be 16

`del`

```
0b101 0 *
```

-> stack should be 0b0

`del`

## Edge case: operations with one preserve base

```
0x10 1 *
```

-> stack should be 0x10

`del`

```
1 0x10 *
```

-> stack should be 16

`del`

```
0b1010 1 /
```

-> stack should be 0b1010

`del`

## Base inheritance with negative numbers

```
0x10 hex neg 5 +
```

-> stack should be -0xb

`del`

```
0b1010 neg 0b11 +
```

-> stack should be -0b111

`del`

```
-10 hex 5 +
```

-> stack should be -0x5

`del`

## Stack operations don't change base

```
0x10 0x20 swap
```

-> stack should be 0x20, 0x10

`del`

```
0x10 20 swap
```

-> stack should be 20, 0x10

`del`

```
0x10 dup
```

-> stack should be 0x10, 0x10

`del`

## Base inheritance with modulo and percent change operations

```
0x64 0x32 mod
```

-> stack should be 0x0

`del`

```
100 50 hex %inv
```

-> stack should be 50

`del`

```
0x64 hex 0x32 %inv
```

-> stack should be 0x32

`del`
