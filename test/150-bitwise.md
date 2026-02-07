# BITWISE OPERATIONS

`default del`

## bitwise not (1)

`0b110 ~`

-> stack should be 0b1

`del`

## bitwise not (2)

`0b1111 ~`

-> stack should be 0b0

`del`

## bitwise not (3)

`0xFF ~`

-> stack should be 0x0

`del`

## bitwise not (4)

`0b10101010 ~`

-> stack should be 0b1010101

`del`

## bitwise not (5)

`255 ~`

-> stack should be 0

`del`

## bitwise or (1)

`0b100000000 0b11 |`

-> stack should be 0b100000011

`del`

## bitwise or (2)

`0xFF 0x0F |`

-> stack should be 0xff

`del`

## bitwise or (3)

`6 3 |`

-> stack should be 7

`del`

## bitwise or (4)

`0 0 |`

-> stack should be 0

`del`

## bitwise and (1)

`6 3 &`

-> stack should be 2

`del`

## bitwise and (2)

`0xFF 0xF0 &`

-> stack should be 0xf0

`del`

## bitwise and (3)

`0b1111 0b1010 &`

-> stack should be 0b1010

`del`

## bitwise and (4)

`255 128 &`

-> stack should be 128

`del`

## bitwise xor (1)

`6 3 ^`

-> stack should be 5

`del`

## bitwise xor (2)

`0xFF 0xFF ^`

-> stack should be 0x0

`del`

## bitwise xor (3)

`0b1010 0b0101 ^`

-> stack should be 0b1111

`del`

## bitwise xor (4)

`100 100 ^`

-> stack should be 0

`del`

## base inheritance (1)

`0xFF 15 &`

-> stack should be 0xf

`del`

## base inheritance (2)

`255 0x0F &`

-> stack should be 15

`del`

## base inheritance (3)

`0b1111 0xFF |`

-> stack should be 0b11111111

`del`

## missing operand

`5 &`

-> error should be 2

`del`

`|`

-> error should be 2

`del`

`~`

-> error should be 2

`del`

## bad operand type

`'hello' 5 &`

-> error should be 3

`del`

`5 'world' |`

-> error should be 3

`del`

`'test' ~`

-> error should be 3

`del`
