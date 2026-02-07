# REAL

`default del`

## real decimal

`1`

-> stack size should be 1

-> stack should be 1

`del`

## real decimal (2)

`2.345`

-> stack should be 2.345

`del`

## real decimal (3)

`1 2.345 3 4.9`

-> stack size should be 4

-> stack should be 1, 2.345, 3, 4.9

`del`

## real inf, nan

```
inf
@inf@
+inf
+@inf@
-inf
-@inf@
nan
@nan@
```

-> stack should be inf, inf, inf, inf, -inf, -inf, nan, nan

`del`

## prec

`default 56 prec pi`

-> stack should be 3.141592653589793

`del 100 prec pi`

-> stack should be 3.14159265358979323846264338328

`del 256 prec pi`

-> stack should be 3.1415926535897932384626433832795028841971693993751058209749445923078164062862

## prec error (1)

`0 prec`

-> error should be 4

`del`

## prec error (2)

`0x8000000000000000 prec`

-> error should be 4

`del default`

## add (1)

`1.2 2.3 +`

-> stack should be 3.5

`del`

## add (2)

`2.3 +`

-> error should be 2

-> stack size should be 1

`del`

## add (3)

`+`

-> error should be 2

`del`

## sub (1)

`1.2 2.3 -`

-> stack should be -1.1

`del`

## sub (2)

`2.3 -`

-> error should be 2

-> stack size should be 1

`del`

## sub (3)

`-`

-> error should be 2

`del`

## mul (1)

`1.2 2.3 *`

-> stack should be 2.76

`del`

## mul (2)

`2.3 *`

-> error should be 2

-> stack size should be 1

`del`

## mul (3)

`*`

-> error should be 2

`del`

## div (1)

`1.2 2.3 /`

-> stack should be 0.52173913043478260869565217391304347826

`del`


## div (2)

`2.3 /`

-> error should be 2

-> stack size should be 1

`del`

## div (3)

`/`

-> error should be 2

`del`

## chs (1)

`3.14 chs`

-> stack should be -3.14

`del`

## chs (2)

`chs`

-> error should be 2

## neg (1)

`3.14 neg`

-> stack should be -3.14

`del`

## neg (2)

`neg`

-> error should be 2

## inv (1)

`2 inv`

-> stack should be 0.5

`del`

## inv (2)

`inv`

-> error should be 2

## % (1)

`2 30 %`

-> stack should be 0.6

`del`

## % (2)

`2 %`

-> error should be 2

-> stack size should be 1

`del`

## % (3)

`%`

-> error should be 2

`del`

## %inv (1)

`2 0.6 %inv`

-> stack should be 30

`del`

## %inv (2)

`2 %inv`

-> error should be 2

-> stack size should be 1

`del`

## %inv (3)

`%inv`

-> error should be 2

`del`

## pow (1)

`2 10 pow`

-> stack should be 1024

`del`

## pow (2)

`2 pow`

-> error should be 2

-> stack size should be 1

`del`

## pow (3)

`pow`

-> error should be 2

`del`

## sqrt (1)

`9 sqrt`

-> stack should be 3

`del`

## sqrt (2)

`sqrt`

-> error should be 2

## sq (1)

`12 sq`

-> stack should be 144

`del`

## sq (2)

`sq`

-> error should be 2

## mod (1)

`9 4 mod`

-> stack should be 1

`del`

## mod (2)

`9 mod`

-> error should be 2

-> stack size should be 1

`del`

## mod (3)

`mod`

-> error should be 2

`del`

## abs (1)

`-9 abs`

-> stack should be 9

`del`

## abs (2)

`9 abs`

-> stack should be 9

`del`

## abs (3)

`abs`

-> error should be 2

`del`

## fact (1)

`6 fact`

-> stack should be 720

`del`

## fact (2)

`'r' fact`

-> error should be 3

-> stack size should be 1

`del`

## fact (3)

`fact`

-> error should be 2

`del`

## sign (1)

`23 sign -34 sign 0 sign`

-> stack should be 1, -1, 0

`del`

## sign (2)

`sign`

-> error should be 2

`del`

## mant (1)

`123.456 mant -123.456 mant 0 mant`

-> stack should be 1.23456, -1.23456, 0

`del`

## mant (2)

`inf mant`

-> error should be 4

`-inf mant`

-> error should be 4

`nan mant`

-> error should be 4

`del`

## xpon (1)

`123.456 xpon -123.456 xpon 0 mant`

-> stack should be 2, 2, 0

`del`

## xpon (2)

`inf xpon`

-> error should be 4

`-inf xpon`

-> error should be 4

`nan xpon`

-> error should be 4

`del`

## min (1)

`1 2 min 4 3 min`

-> stack should be 1, 3

`del`

## min (2)

`'a' 'g' min`

-> error should be 3

`del`

## min (3)

`1 min`

-> error should be 2

`del`

## min (4)

`min`

-> error should be 2

`del`

## max (1)

`1 2 max 4 3 max`

-> stack should be 2, 4

`del`

## max (2)

`'a' 'g' max`

-> error should be 3

`del`

## max (3)

`1 max`

-> error should be 2

`del`

## max (4)

`max`

-> error should be 2

`del`

## ip (1)

`1.22 ip`

-> stack should be 1

`del`

## ip (2)

`-1.22 ip`

-> stack should be -1

`del`

## fp (1)

`1.22 fp`

-> stack should be 0.22

`del`

## fp (2)

`-1.22 fp`

-> stack should be -0.22

`del`

## floor (1)

`1.22 floor`

-> stack should be 1

`del`

## floor (2)

`-1.22 floor`

-> stack should be -2

`del`

## ceil (1)

`1.22 ceil`

-> stack should be 2

`del`

## ceil (2)

`-1.22 ceil`

-> stack should be -1

`del`

## round (1) - positive rounds down

`2.4 round`

-> stack should be 2

`del`

## round (2) - positive rounds up

`2.6 round`

-> stack should be 3

`del`

## round (3) - positive half rounds up

`2.5 round`

-> stack should be 3

`del`

## round (4) - negative rounds toward zero

`-2.4 round`

-> stack should be -2

`del`

## round (5) - negative rounds away from zero

`-2.6 round`

-> stack should be -3

`del`

## round (6) - negative half rounds away from zero

`-2.5 round`

-> stack should be -3

`del`

## round (7) - already integer

`5 round`

-> stack should be 5

`del`

## round (8) - negative already integer

`-7 round`

-> stack should be -7

`del`

## round (9) - zero

`0 round`

-> stack should be 0

`del`

## round (10) - nan

`nan round`

-> stack should be nan

`del`

## round (11) - inf

`inf round`

-> stack should be inf

`del`

## round (12) - negative inf

`-inf round`

-> stack should be -inf

`del`

## round (13) - complex

`(1.6,2.4) round`

-> stack should be (2,2)

`del`

## round (14) - complex with negative

`(-1.6,-2.4) round`

-> stack should be (-2,-2)

`del`

## round (15) - complex half

`(1.5,-2.5) round`

-> stack should be (2,-3)

`del`

## floor (3) - complex

`(1.6,2.4) floor`

-> stack should be (1,2)

`del`

## floor (4) - complex with negative

`(-1.6,-2.4) floor`

-> stack should be (-2,-3)

`del`

## ceil (3) - complex

`(1.6,2.4) ceil`

-> stack should be (2,3)

`del`

## ceil (4) - complex with negative

`(-1.6,-2.4) ceil`

-> stack should be (-1,-2)

`del`

## sticky operator + (1)

`1 2+`

-> stack should be 3

`del`

## sticky operator + (2)

`1 2+ 3+`

-> stack should be 6

`del`

## sticky operator -

`10 3-`

-> stack should be 7

`del`

## sticky operator *

`4 5*`

-> stack should be 20

`del`

## sticky operator /

`20 4/`

-> stack should be 5

`del`

## sticky operator %

`200 25%`

-> stack should be 50

`del`

## sticky operator & (1)

`0b1111 0b1010&`

-> stack should be 0b1010

`del`

## sticky operator & (2)

`0xff 0x0f&`

-> stack should be 0xf

`del`

## sticky operator |

`0b1100 0b0011|`

-> stack should be 0b1111

`del`

## sticky operator ^

`0b1111 0b1010^`

-> stack should be 0b101

`del`

## sticky operator ~ (1)

`0b1010~`

-> stack should be 0b101

`del`

## sticky operator ~ (2)

`0b1111~`

-> stack should be 0b0

`del`

## sticky chained operators

`2 3 4+*`

-> stack should be 14

`del`

## sticky with negative number

`-5 3+`

-> stack should be -2

`del`

## sticky with scientific notation

`1e5 2*`

-> stack should be 200000

`del default`
