# LOGICAL TESTS

`default del`

## and (1)

`and`

-> error should be 2

`1 and`

-> error should be 2

`del`

## and (2)

`0 0 and`

-> stack size should be 1

-> stack should be 0

`del`

## and (3)

`1 0 and`

-> stack size should be 1

-> stack should be 0

`del`

## and (4)

`0 1 and`

-> stack size should be 1

-> stack should be 0

`del`

## and (5)

`1 1 and`

-> stack size should be 1

-> stack should be 1

`del`

## or (1)

`or`

-> error should be 2

`1 or`

-> error should be 2

`del`

## or (2)

`0 0 or`

-> stack size should be 1

-> stack should be 0

`del`

## or (3)

`0 1 or`

-> stack size should be 1

-> stack should be 1

`del`

## or (4)

`1 0 or`

-> stack size should be 1

-> stack should be 1

`del`

## or (5)

`1 1 or`

-> stack size should be 1

-> stack should be 1

`del`

## xor (1)

`xor`

-> error should be 2

`1 xor`

-> error should be 2

`del`

## xor (2)

`0 0 xor`

-> stack size should be 1

-> stack should be 0

`del`

## xor (3)

`0 1 xor`

-> stack size should be 1

-> stack should be 1

`del`

## xor (4)

`1 0 xor`

-> stack size should be 1

-> stack should be 1

`del`

## xor (5)

`1 1 xor`

-> stack size should be 1

-> stack should be 0

`del`

## not (1)

`not`

-> error should be 2

`1 not`

-> stack size should be 1

-> stack should be 0

`del`

## not (2)

`0 not`

-> stack size should be 1

-> stack should be 1

`del`

## >

`0 0.1 >`

-> stack should be 0

`del`

`0.1 0 >`

-> stack should be 1

`del`

`1 1 >`

-> stack should be 0

`del`

## >=

`0 0.1 >=`

-> stack should be 0

`del`

`0.1 0 >=`

-> stack should be 1

`del`

`1 1 >=`

-> stack should be 1

`del`

## <

`0 0.1 <`

-> stack should be 1

`del`

`0.1 0 <`

-> stack should be 0

`del`

`1 1 <`

-> stack should be 0

`del`

## <=

`0 0.1 <=`

-> stack should be 1

`del`

`0.1 0 <=`

-> stack should be 0

`del`

`1 1 <=`

-> stack should be 1

`del`

## !=

`0 0.1 !=`

-> stack should be 1

`del`

`1 1 !=`

-> stack should be 0

`del`

## ==

`0 0.1 ==`

-> stack should be 0

`del`

`1 1 ==`

-> stack should be 1

`del`

## same

`0 0.1 same`

-> stack should be 0

`del`

`1 1 same`

-> stack should be 1

`del`

## complex !=

`(1,2) (3,4) !=`

-> stack should be 1

`del`

`(1,2) (1,0) !=`

-> stack should be 1

`del`

`(1,2) (0,2) !=`

-> stack should be 1

`del`

`(1,2) (1,2) !=`

-> stack should be 0

`del`

## complex ==

`(1,2) (3,4) ==`

-> stack should be 0

`del`

`(1,2) (1,0) ==`

-> stack should be 0

`del`

`(1,2) (0,2) ==`

-> stack should be 0

`del`

`(1,2) (1,2) ==`

-> stack should be 1

`del`

## complex same

`(1,2) (3,4) same`

-> stack should be 0

`del`

`(1,2) (1,0) same`

-> stack should be 0

`del`

`(1,2) (0,2) same`

-> stack should be 0

`del`

`(1,2) (1,2) same`

-> stack should be 1

`del`
