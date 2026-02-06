# PARSE HEX, BIN, BASE

`default del`

## hex (1)

`0x10 0X10`

-> stack should be 0x10, 0x10

`del`

## hex (2)

`-0x10 0x-10`

-> stack should be -0x10, -0x10

`del`

## hex conversion from dec bin

`12 hex`

-> stack should be 0xc

`del`

`0b100000000000000000000000000000000000000000000000000 hex`

-> stack should be 0x4000000000000

`del`

## hex no fractional part

`12.34 hex`

-> stack should be 0xc

`dec`

-> stack should be 12.34

`del`

## hex display mode invariance

`12.34 hex`

`4 std dup`

`4 fix dup`

`4 sci dup`

`default`

-> stack should be 0xc, 0xc, 0xc, 0xc

`del`

## hex err

`0x 0X`

-> stack should be '0x', '0X'

`del`

## bin (1)

`0b10 0B10`

-> stack should be 0b10, 0b10

`del`

## bin (2)

`-0b10 0b-10`

-> stack should be -0b10, -0b10

`del`

## bin conversion from dec hex

`12 bin`

-> stack should be 0b1100

`del`

`0x4000000000000 bin`

-> stack should be 0b100000000000000000000000000000000000000000000000000

`del`

## bin no fractional part

`12.34 bin`

-> stack should be 0b1100

`dec`

-> stack should be 12.34

`del`

## bin display mode invariance

`12.34 bin`

`4 std dup`

`4 fix dup`

`4 sci dup`

`default`

-> stack should be 0b1100, 0b1100, 0b1100, 0b1100

`del`

## bin err

`0b 0B`

-> stack should be '0b', '0B'

`del`

## base (1)

`3b10 3B10`

-> stack should be 3b10, 3b10

`del`

## base (2)

`-3b10 3b-10`

-> stack should be -3b10, -3b10

`del`

## base conversion from dec

`12 3 base`

-> stack should be 3b110

`del`

## base no fractional part

`12.34 3 base`

-> stack should be 3b110

`dec`

-> stack should be 12.34

`del`

## base display mode invariance

`12.34 3 base`

`4 std dup`

`4 fix dup`

`4 sci dup`

`default`

-> stack should be 3b110, 3b110, 3b110, 3b110

`del`

## wrong base errors

`3 0 base`

-> error should be 4

`del 3 63 base`

-> error should be 4

`del`

## base aliases

`del 10b12345 2b1100 16b1234`

-> stack should be 12345, 0b1100, 0x1234

## dec (1)

`2 50 pow`

-> stack should be 1125899906842624

`del default`

## dec from hex bin

`0x4000000000000 dec`

`0b100000000000000000000000000000000000000000000000000 dec`

-> stack should be 1125899906842624, 1125899906842624

`del default`

## base display (1)

`2 62 for i 62 i base next`

-> stack should be 0b111110, 3b2022, 4b332, 5b222, 6b142, 7b116, 8b76, 9b68, 62, 11b57, 12b52, 13b4a, 14b46, 15b42, 0x3e, 17b3b, 18b38, 19b35, 20b32, 21b2k, 22b2i, 23b2g, 24b2e, 25b2c, 26b2a, 27b28, 28b26, 29b24, 30b22, 31b20, 32b1u, 33b1t, 34b1s, 35b1r, 36b1q, 37b1P, 38b1O, 39b1N, 40b1M, 41b1L, 42b1K, 43b1J, 44b1I, 45b1H, 46b1G, 47b1F, 48b1E, 49b1D, 50b1C, 51b1B, 52b1A, 53b19, 54b18, 55b17, 56b16, 57b15, 58b14, 59b13, 60b12, 61b11, 62b10

`del default`

## base display (2)

`2 62 for i i 62 base next`

-> stack should be 62b2, 62b3, 62b4, 62b5, 62b6, 62b7, 62b8, 62b9, 62bA, 62bB, 62bC, 62bD, 62bE, 62bF, 62bG, 62bH, 62bI, 62bJ, 62bK, 62bL, 62bM, 62bN, 62bO, 62bP, 62bQ, 62bR, 62bS, 62bT, 62bU, 62bV, 62bW, 62bX, 62bY, 62bZ, 62ba, 62bb, 62bc, 62bd, 62be, 62bf, 62bg, 62bh, 62bi, 62bj, 62bk, 62bl, 62bm, 62bn, 62bo, 62bp, 62bq, 62br, 62bs, 62bt, 62bu, 62bv, 62bw, 62bx, 62by, 62bz, 62b10

`del default`

## base display (3)

```
100 dup 3 base ==
13455600 dup 5 base ==
55756 dup 17 base ==
2345321 dup 62 base ==
```

-> stack should be 1, 1, 1, 1

`del default`

## base display (4)

```
100 18 base dup 3 base == dec
13455600 55 base dup 5 base == dec
55756 9 base dup 17 base == dec
2345321 57 base dup 62 base == dec
```

-> stack should be 1, 1, 1, 1

`del default`

## negative base numbers (1)

`1000 hex neg`

-> stack should be -0x3e8

`del default`

## negative base numbers (2)

`1000 7 base neg`

-> stack should be -7b2626

`del default`

## negative base numbers (3)

`1000 bin neg`

-> stack should be -0b1111101000

`del default`

## negative base numbers (4)

`-0b1111101000 3 base`

-> stack should be -3b1101001

`del default`

## base on complexes

```
(0b110,0x102) dup bin swap dup hex dup 5 base
```

-> stack should be (0b110,0b100000010), (0b110,0x102), (0x6,0x102), (5b11,5b2013)

`del default`

## inf should not be based-represented

`-1 bin 0 bin / 1 3 base 0 3 base /`

-> stack should be -inf, inf

`del default`

## nan should not be based-represented

`-0 bin 0 bin / 0 3 base 0 3 base /`

-> stack should be nan, nan

`del default`

## complex base 16

`(0x10,0x20)`

-> stack should be (0x10,0x20)

## complex base 16 (2)

`del (16,32) hex`

-> stack should be (0x10,0x20)

## complex base 2

`del (0b111,0b110)`

-> stack should be (0b111,0b110)

## complex base 2 (2)

`del (7,6) bin`

-> stack should be (0b111,0b110)

## complex multiple bases re / im

`del (0x10,0b111) dup dup hex swap dec`

-> stack should be (0x10,0b111), (0x10,0x7), (16,7)
