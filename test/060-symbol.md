# SYMBOL

`default del`

## symbol + symbol

`'hello' 'world' +`

-> stack should be 'helloworld'

`del`

## symbol + symbol (2)

`'foo' 'bar' + 'baz' +`

-> stack should be 'foobarbaz'

`del`

## symbol + empty symbol

`'hello' '' +`

-> stack should be 'hello'

`del`

## empty symbol + symbol

`'' 'world' +`

-> stack should be 'world'

`del`

## empty symbol + empty symbol

`'' '' +`

-> stack should be ''

`del`

## symbol + number (decimal)

`'val=' 42 +`

-> stack should be 'val=42'

`del`

## symbol + number (negative)

`'result:' -123 +`

-> stack should be 'result:-123'

`del`

## symbol + number (float)

`'pi=' 3.14159 +`

-> stack should be 'pi=3.14159'

`del`

## symbol + number (hex)

`'addr=' 0xFF +`

-> stack should be 'addr=0xff'

`del`

## symbol + number (binary)

`'bits=' 0b1010 +`

-> stack should be 'bits=0b1010'

`del`

## symbol + number (base)

`'base8=' 8b77 +`

-> stack should be 'base8=8b77'

`del`

## number + symbol

`42 'is the answer' +`

-> stack should be '42is the answer'

`del`

## number (hex) + symbol

`0xDEAD '_BEEF' +`

-> stack should be '0xdead_BEEF'

`del`

## number (binary) + symbol

`0b11 '_bits' +`

-> stack should be '0b11_bits'

`del`

## symbol + complex

`'z=' (1,2) +`

-> stack should be 'z=(1,2)'

`del`

## complex + symbol

`(3,4) 'i' +`

-> stack should be '(3,4)i'

`del`

## symbol + complex (negative imag)

`'complex:' (1,-2) +`

-> stack should be 'complex:(1,-2)'

`del`

## empty symbol + number

`'' 123 +`

-> stack should be '123'

`del`

## number + empty symbol

`456 '' +`

-> stack should be '456'

`del`

## empty symbol + complex

`'' (1,1) +`

-> stack should be '(1,1)'

`del`

## complex + empty symbol

`(2,3) '' +`

-> stack should be '(2,3)'

`del`

## error: symbol + program

`'sym' << 1 2 + >> +`

`error`

-> stack should be 'sym', « 1 2 + », 3

`del`

## error: program + symbol

`<< 1 >> 'sym' +`

`error`

-> stack should be « 1 », 'sym', 3

`del`

## error: number + program

`42 << 1 >> +`

`error`

-> stack should be 42, « 1 », 3

`del`

## error: program + number

`<< 1 >> 42 +`

`error`

-> stack should be « 1 », 42, 3

`del`

## error: complex + program

`(1,2) << 1 >> +`

`error`

-> stack should be (1,2), « 1 », 3

`del`

## error: program + complex

`<< 1 >> (1,2) +`

`error`

-> stack should be « 1 », (1,2), 3

`del`

## error: program + program

`<< 1 >> << 2 >> +`

`error`

-> stack should be « 1 », « 2 », 3

`del`

## symbol == symbol (equal)

`'hello' 'hello' ==`

-> stack should be 1

`del`

## symbol == symbol (not equal)

`'hello' 'world' ==`

-> stack should be 0

`del`

## symbol == symbol (empty symbols equal)

`'' '' ==`

-> stack should be 1

`del`

## symbol == symbol (case sensitive)

`'Hello' 'hello' ==`

-> stack should be 0

`del`

## symbol same symbol (equal)

`'test' 'test' same`

-> stack should be 1

`del`

## symbol same symbol (not equal)

`'foo' 'bar' same`

-> stack should be 0

`del`

## symbol != symbol (equal)

`'hello' 'hello' !=`

-> stack should be 0

`del`

## symbol != symbol (not equal)

`'hello' 'world' !=`

-> stack should be 1

`del`

## symbol != symbol (empty vs non-empty)

`'' 'x' !=`

-> stack should be 1

`del`

## error: symbol == number

`'hello' 42 ==`

`error`

-> stack should be 'hello', 42, 3

`del`

## error: number == symbol

`42 'hello' ==`

`error`

-> stack should be 42, 'hello', 3

`del`

## error: symbol == complex

`'hello' (1,2) ==`

`error`

-> stack should be 'hello', (1,2), 3

`del`

## error: symbol == program

`'hello' << 1 >> ==`

`error`

-> stack should be 'hello', « 1 », 3

`del`

## error: symbol != number

`'hello' 42 !=`

`error`

-> stack should be 'hello', 42, 3

`del`

## error: number != symbol

`42 'hello' !=`

`error`

-> stack should be 42, 'hello', 3

`del`

## error: symbol != complex

`'hello' (1,2) !=`

`error`

-> stack should be 'hello', (1,2), 3

`del`

## error: symbol != program

`'hello' << 1 >> !=`

`error`

-> stack should be 'hello', « 1 », 3

`del`
