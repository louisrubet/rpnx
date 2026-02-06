# STACK

`default del`

## entry depth (1)

`1 depth`

-> stack size should be 2

## entry depth (2)

`1 del depth`

-> stack should be 0

`del`

## swap

`1 2 swap`

-> stack size should be 2

-> stack should be 2, 1

`del`

## swap with filled stack

`5 6 1 2 swap`

-> stack size should be 4

-> stack should be 5, 6, 2, 1

`del`

## swap error

`5 swap`

-> stack size should be 1

-> error should be 2

`del`

## drop

`1 2 3 drop`

-> stack size should be 2

-> stack should be 1, 2

`del`

## drop2

`1 2 3 drop2`

-> stack size should be 1

-> stack should be 1

`del`

## drop error

`drop`

-> error should be 2

`del`

## drop2 error (1)

`drop2`

-> error should be 2

`del`

## drop2 error (2)

`1 drop2`

-> error should be 2

`del`

## test dup

`1 dup`

-> stack size should be 2

-> stack should be 1, 1

`del`

## test dup2

`1 2 dup2`

-> stack size should be 4

-> stack should be 1, 2, 1, 2

`del`

## test rot

`1 2 3 rot`

-> stack size should be 3

-> stack should be 2, 3, 1

`del`

## test rot 2

`5 6 7 rot rot`

-> stack should be 7, 5, 6

`del`

## test rot with filled stack

`5 6 1 2 3 rot`

-> stack size should be 5

-> stack should be 5, 6, 2, 3, 1

`del`

## test depth

`1 2 3`

`depth`

-> stack size should be 4

-> stack should be 1, 2, 3, 3

`del`

## test pick

`1 2 3 4 2 pick`

-> stack size should be 5

-> stack should be 1, 2, 3, 4, 3

`0 pick`

-> error should be 4

`7 pick`

-> error should be 4

`erase`

## test erase

-> stack size should be 0

`erase`

## test del

`1 2 3 4 5`

`del`

-> stack size should be 0

## test dropn

`1 2 2 dropn`

-> stack size should be 0

`del`
 
## test dropn error

`1 2 3 dropn`

-> stack size should be 3

-> error should be 2

`del`
 
## test dupn

`1 2 3 4 3 dupn`

-> stack should be 1, 2, 3, 4, 2, 3, 4

`del`

## test dupn error

`1 2 3 4 5 dupn`

-> stack size should be 4

-> error should be 2

`del`

## test roll

`1 2 3 4 5 4 roll`

-> stack should be 1, 3, 4, 5, 2

`del`

## test roll with filled stack

`10 11 1 2 3 4 5 4 roll`

-> stack should be 10, 11, 1, 3, 4, 5, 2

`del`

## test roll error

`1 2 3 4 5 6 roll`

-> stack size should be 5

-> error should be 2

`del`

## test rolld

`10 20 30 40 50 3 rolld`

-> stack should be 10, 20, 50, 30, 40

`del`

## test rolld with filled stack

`80 90 10 20 30 40 50 3 rolld`

-> stack should be 80, 90, 10, 20, 50, 30, 40

`del`

## test rolld error

`1 2 3 4 5 6 rolld`

-> stack size should be 5

-> error should be 2

`del`

## test over

`3.14 15.16 over`

-> stack should be 3.14, 15.16, 3.14

`del`

## test over error

`2 over`

-> stack size should be 1

-> error should be 2

`del`
