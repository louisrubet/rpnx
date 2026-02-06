# TRIGONOMETRY

`del default`

## pi

`6 fix 128 prec`

`pi`

-> stack should be 3.141593

`del`

## d->r

`180 d->r pi ==`

-> stack should be 1.000000

`del`

## r->d

`pi r->d 180 ==`

-> stack should be 1.000000

`del`

## sin asin

`0 sin pi 2 / sin pi 6 / sin`
`0 asin 0 == 1 asin pi 2 / == 0.5 asin pi 6 / ==`

-> stack should be 0.000000, 1.000000, 0.500000, 1.000000, 1.000000, 1.000000

`del`

## cos acos

`0 cos pi 3 / cos`
`1 acos 0 == 0.5 acos pi 3 / ==`

-> stack should be 1.000000, 0.500000, 1.000000, 1.000000

`del`

## tan atan

`pi 4 / tan 1 == 1 atan pi 4 / ==`

-> stack should be 1.000000, 1.000000

`del`

## sin asin

`(1,2) sin (3.165779,1.959601) asin`

-> stack should be (3.165779,1.959601), (1.000000,2.000000)

`del`

## cos acos

`(1,2) cos (2.032723,-3.051898) acos`

-> stack should be (2.032723,-3.051898), (1.000000,2.000000)

`del`

## tan atan

`(1,2) tan (0.033813,1.014794) atan`

-> stack should be (0.033813,1.014794), (1.000004,1.999996)

`del`

## sin asin cos acos tan atan error

`'ok' sin`

-> error should be 3

`asin`

-> error should be 3

`cos`

-> error should be 3

`acos`

-> error should be 3

`tan`

-> error should be 3

`atan`

-> error should be 3

`del`
