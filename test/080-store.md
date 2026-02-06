# STORE

`default del`

## symbol entry

`'test'`

-> stack size should be 1

-> stack should be 'test'

`del`

## symbol entry (2)

`'test`

-> stack size should be 1

-> stack should be 'test'

`del`

## symbol entry (3)

`''`

-> stack size should be 1

-> stack should be ''

`del`

## symbol entry (4)

`'`

-> stack size should be 1

-> stack should be ''

`del`

## sto (1)

`1 'a' sto`

-> stack size should be 0

`'a' a`

-> stack should be 'a', 1

`del`

## sto (2)

`2 'a' sto a`

-> stack should be 2

`del`

## sto (3)

`3 'b' sto b`

-> stack should be 3

`del`

## rcl (1)

`'a' rcl`

-> stack should be 2

`del`

## rcl (2)

`'b' rcl`

-> stack should be 3

`del`

## rcl (3)

`'var' rcl`

-> error should be 5

-> stack should be 'var'

`del`

## sto in prog then rcl

`3 << 'r' sto >> eval r 'r' rcl`

-> stack should be 3, 3

`del`

## purge (1)

`a 'a' purge a`

-> stack should be 2, 'a'

`del`

## purge (2)

`'a' purge`

-> error should be 5

-> stack size should be 0

`del`

## purge (3)

`3 'a' sto a 'a' purge`

-> stack should be 3

`del`

## sto+ (1)

`8 'a' sto 2 'a' sto+ a`

-> stack should be 10

`del`

## sto+ (2)

`'a' sto+`

-> stack size should be 1

-> error should be 2

`del`

## sto+ (3)

`3 'zz' sto+`

-> stack should be 3, 'zz'

-> error should be 5

`del`

## sto- (1)

`2 'a' sto`

`2 'a' sto-`

`a`

-> stack should be 0

`del`

## sto- (2)

`'a' sto-`

-> stack size should be 1

-> error should be 2

`del`

## sto- (3)

`3 'zz' sto-`

-> stack should be 3, 'zz'

-> error should be 5

`del`

## sto* (1)

`2 'a' sto`

`3 'a' sto*`

`a`

-> stack should be 6

`del`

## sto* (2)

`'a' sto*`

-> stack size should be 1

-> error should be 2

`del`

## sto* (3)

`3 'zz' sto*`

-> stack should be 3, 'zz'

-> error should be 5

`del`

## sto/ (1)

`2 'a' sto`

`4 'a' sto/`

`a`

-> stack should be 0.5

`del`

## sto/ (2)

`'a' sto/`

-> stack size should be 1

-> error should be 2

`del`

## sto/ (4)

`3 'zz' sto/`

-> stack should be 3, 'zz'

-> error should be 5

`del`

## sneg (1)

`7 'a' sto`

`'a' sneg`

`a`

-> stack should be -7

`del`

## sneg (2)

`sneg`

-> error should be 2

`del`

## sneg (3)

`'zz' sneg`

-> error should be 5

-> stack should be 'zz'

`del`

## sinv (1)

`4 'a' sto`

`'a' sinv`

`a`

-> stack should be 0.25

`del`

## sinv (2)

`sinv`

-> error should be 2

`del`

## sinv (3)

`'zz' sinv`

-> error should be 5

-> stack should be 'zz'

`del`

## clusr

`3.14 'abc' sto clusr 'abc' rcl`

-> error should be 5

-> stack should be 'abc'

`del`
