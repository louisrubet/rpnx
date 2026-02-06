# COMPLEX

`default del`

## entry (1)

`(1,2) ( 1,2) (1 ,2) (1, 2) (1,2 )`

-> stack should be (1,2), (1,2), (1,2), (1,2), (1,2)

`del`

## entry (2)

`(1.3,2.444555`

-> stack should be (1.3,2.444555)

`del`

## entry (3)

`(1,`

-> stack should be '(1,'

`del`

## entry (4)

`(nan,+inf)`

-> stack should be (nan,inf)

`del`

## entry (5)

`(0x1234,0x1010)`

-> stack should be (0x1234,0x1010)

`del`

## entry (6)

`(0b11,0b101)`

-> stack should be (0b11,0b101)

`del`

## add (1)

`(1.2,2.3) (1,2) +`

-> stack should be (2.2,4.3)

`del`

## add (2)

`(1.2,2.3) 3 +`

-> stack should be (4.2,2.3)

`del`

## add (3)

`3 (1.2,2.3) +`

-> stack should be (4.2,2.3)

`del`

## sub (1)

`(1.2,2.3) (2,2) -`

-> stack should be (-0.8,0.3)

`del`

## sub (2)

`(1.2,2.3) 1 -`

-> stack should be (0.2,2.3)

`del`

## sub (3)

`1 (1.2,2.3) -`

-> stack should be (-0.2,-2.3)

`del`

## mul (1)

`(1,2) (3,4) *`

-> stack should be (-5,10)

`del`

## mul (2)

`(3,4) (1,2) *`

-> stack should be (-5,10)

`del`

## mul (3)

`(3,4) 2 *`

-> stack should be (6,8)

`del`

## mul (3)

`2 (3,4) *`

-> stack should be (6,8)

`del`

## div (1)

`(1,2) (3,4) /`

-> stack should be (0.44,0.08)

`del`

## div (2)

`(1,2) 2 /`

-> stack should be (0.5,1)

`del`

## div (3)

`2 (3,4) /`

-> stack should be (0.24,-0.32)

`del`

## re (1)

`(1.2,3.4) re`

-> stack should be 1.2

`del`

## re (2)

`3 re`

-> stack should be 3

`del`

## re (3)

`re`

-> error should be 2

`del`

## im (1)

`(1.2,3.4) im`

-> stack should be 3.4

`del`

## im (2)

`3 im`

-> stack should be 0

`del`

## im (3)

`im`

-> error should be 2

`del`

## chs

`(3.14,6.28) chs`

-> stack should be (-3.14,-6.28)

`del`

## neg

`(-3.14,-6.28) neg`

-> stack should be (3.14,6.28)

`del`

## inv (1)

`(2,4) inv`

-> stack should be (0.1,-0.2)

`del`

## inv (2)

`(0.1,-0.2) inv`

-> stack should be (2,4)

`del`

## abs

`(3,4) abs`

-> stack should be 5

`del`

## sign (1)

`(1,0) sign`

-> stack should be (1,0)

`del`

## sign (2)

`(0,1) sign`

-> stack should be (0,1)

`del`

## sign (3)

`(3,-4) sign`

-> stack should be (0.6,-0.8)

`del`

## sq (1)

`(12,10) sq`

-> stack should be (44,240)

`del`

## sqr (1)

`(12,10) sq`

-> stack should be (44,240)

`del`

## arg (1)

`(1,1) arg pi 4 / ==`

-> stack should be 1

`del`

## arg (2)

`1000 prec`

`(-1,1) arg pi 3 * 4 / ==`

-> stack should be 1

`default del`

## arg (3)

`1000 prec`

`(1,-1) arg pi chs 4 / ==`

-> stack should be 1

`default del`

## arg (4)

`1000 prec`

`(-1,-1) arg pi -3 * 4 / ==`

-> stack should be 1

`default del`

## arg (5)

`1000 prec`

`(1,0) arg 0 ==`

-> stack should be 1

`default del`

## arg (6)

`1000 prec`

`(0,1) arg pi 2 / ==`

-> stack should be 1

`default del`

## arg (7)

`1000 prec`

`(-1,0) arg pi ==`

-> stack should be 1

`default del`

## arg (8)

`1000 prec`

`(0,-1) arg pi neg 2 / ==`

-> stack should be 1

`default del`

## conj

`(1,2) conj dup conj`

-> stack should be (1,-2), (1,2)

`del`

## r->c (1)

`1 2 r->c`

-> stack should be (1,2)

`del`

## r->c (2)

`0x12 0b1101 r->c`

-> stack should be (0x12,0b1101)

`del`

## r->c error (1)

`1 r->c`

-> error should be 2

`del`

## r->c error (2)

`r->c`

-> error should be 2

`del`

## r->c error (3)

`'1' '2' r->c`

-> error should be 3

`del`

## c->r (1)

`(1,2) c->r`

-> stack should be 1, 2

`del`

## c->r (2)

`(0x12,0b1101) c->r`

-> stack should be 0x12, 0b1101

`del`

## c->r error (2)

`c->r`

-> error should be 2

`del`

## c->r error (2)

`'4' c->r`

-> error should be 3

`del`

## r->p (1)

`6 fix`

`(1,2) r->p`

-> stack should be (2.236068,1.107149)

`del`

## r->p (2)

`19 fix`

`1 r->p`

-> error should be 3.000000

`del`

## r->p (3)

`r->p`

-> error should be 2

`del`

## r->p (4)

`'1' '2' r->p`

-> error should be 3

`del`

## pow (1)

`6 fix`

`(1,2) 2 pow`

-> stack should be (-3.000000,4.000000)

`del`

## pow (2)

`(1,2) 4 pow`

-> stack should be (-7.000000,-24.000000)

`del`

## pow (3)

`-3 .2 pow`

-> stack should be (1.007818,0.732222)

`del`

## pow (4)

`-3 (1,2) pow`

-> stack should be (0.003284,-0.004539)

`del`

## pow (5)

`(1,2) (1,2) pow`

-> stack should be (-0.222517,0.100709)

`del`

## sqrt (1)

`(3,4) sqrt`

-> stack should be (2.000000,1.000000)

`del`

## sqrt (2)

`-3 sqrt`

-> stack should be (0.000000,1.732051)

`del`

`default`
