# LOGARITHMS

`del default 6 fix`

## euler constant

`e`

-> stack should be 0.577216

`del`

## ln log exp

`2 ln exp`

`2 log exp`

-> stack should be 2.000000, 2.000000

`del`

## lnp1 expm

`4 lnp1 expm`

-> stack should be 4.000000

`del`


## log10 alog10 exp10

`5 log10 alog10`

`5 log10 exp10`

-> stack should be 5.000000, 5.000000

`del`

## log2 alog2 exp2

`6 log2 alog2`

`6 log2 exp2`

-> stack should be 6.000000, 6.000000

`del`

## sinh asinh

`1 sinh asinh`

-> stack should be 1.000000

`del`

## cosh acosh

`1.1 cosh acosh`

-> stack should be 1.100000

`del`

## tanh atanh

`1.2 tanh atanh`

-> stack should be 1.200000

`del`

## complex ln exp

`(-1,-2) ln exp`

-> stack should be (-1.000000,-2.000000)

`del`

## complex log exp

`(1,2) log exp`

-> stack should be (1.000000,2.000000)

`del`

## complex lnp1 expm

`(1,2) lnp1 expm`

-> stack should be (1.000000,2.000000)

`del`

## complex log10 alog10

`(1,-2) log10 alog10`

-> stack should be (1.000000,-2.000000)

`del`


## complex log2 alog2

`(-1,-2) log2 alog2`

-> stack should be (-1.000000,-2.000000)

`del`

## complex sinh asinh

`(1,1.5) sinh asinh`

-> stack should be (1.000000,1.500000)

`del`

## complex cosh acosh

`(1,2) cosh acosh`

-> stack should be (1.000000,2.000000)

`del`

## complex tanh atanh

`(1,1.5) tanh atanh`

-> stack should be (1.000000,1.500000)

`del`

## ln log lnp1 exp expm error

`'ok' ln`

-> error should be 3

`log`

-> error should be 3

`lnp1`

-> error should be 3

`exp`

-> error should be 3

`expm`

-> error should be 3

`del`

## log10 alog10 exp10 log2 alog2 exp2 error

`'ok' log10`

-> error should be 3

`alog10`

-> error should be 3

`exp10`

-> error should be 3

`log2`

-> error should be 3

`alog2`

-> error should be 3

`exp2`

-> error should be 3

`del`

## sinh asinh cosh acosh tanh atanh error

`'ok' sinh`

-> error should be 3

`asinh`

-> error should be 3

`cosh`

-> error should be 3

`acosh`

-> error should be 3

`tanh`

-> error should be 3

`atanh`

-> error should be 3

`del`
