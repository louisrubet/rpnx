# PARSE COMPLEX

`default del`

## type

`(1,2) type`

-> stack should be "complex"

`del`

## cplx

`(1,2) (1, 2) ( 1 , 2 )`

-> stack should be (1,2), (1,2), (1,2)

`del`

## cplx inf nan

```
(inf,3)
(-inf,nan) (inf,-inf)
```

-> stack should be (inf,3), (-inf,nan), (inf,-inf)

`del`

## unterminated

`( 3.14e2 , -2`

-> stack should be (314,-2)

`del`

## unterminated 2

`(-inf, nan`

-> stack should be (-inf,nan)

`del`

## unterminated err

```
(
(a
(123
(,
(,)
(12,
(,13)
(,3.14
```

-> stack should be '(', '(a', '(123', '(,', '(,)', '(12,', '(,13)', '(,3.14'

`del`
