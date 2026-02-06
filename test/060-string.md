# STRING

`default del`

## string entry

`"test string"`

-> stack size should be 1

-> stack should be "test string"

`del`

## string entry (2)

`"test string`

-> stack size should be 1

-> stack should be "test string"

`del`

## string entry (3)

`"`

-> stack size should be 1

-> stack should be ""

`del`

## ->str on real (1)

`1 ->str`

-> stack should be "1"

`del`

## ->str on real (2)

`1.234 ->str`

-> stack should be "1.234"

`del`

## ->str on real (3)

`1.234 20 fix ->str`

-> stack should be "1.23400000000000000000"

`del`

## ->str on symbol (1)

`toto ->str`

-> stack should be "'toto'"

`del`

## ->str on symbol (2)

`'toto' ->str`

-> stack should be "'toto'"

`del default`

## str-> on real (1)

`"1" str->`

-> stack should be 1

`del`

## str-> on real (2)

`"1 2.345 3 4.9" str->`

-> stack should be 1, 2.345, 3, 4.9

`del`

## str-> on real (3)

`4 fix "1 2.345 3 4.9" str->`

-> stack should be 1.0000, 2.3450, 3.0000, 4.9000

`del default`

## str-> on constant (1)

`"pi" str->`

-> stack should be 3.1415926535897932384626433832795028842

`del`

## str-> on constant (2)

`"'pi' 'e'" str->`

-> stack should be 'pi', 'e'

`del`

## str-> on command (1)

`"2 dup" str->`

-> stack should be 2, 2

`del`

## str-> on command (2)

`"3.14 my_pi sto" str->`

-> stack size should be 0

`my_pi`

-> stack should be 3.14

`del`

## str-> on program

`"<< -> n << n >> >>" str->`

-> stack should be « -> n « n » »

`del`

## add (1)

`12 34 "one" "two" +`

-> stack should be 12, 34, "onetwo"

`del`

## add (2)

`"" "one" + "two" "" +`

-> stack should be "one", "two"

`del`

## add (3)

`"one" +`

-> stack size should be 1

-> error should be 2

`del`

## chr (1)

`"" 33 40 for i i chr + next`

-> stack should be "!"#$%&'("

`del`

## chr (2)

`-223 chr 0 chr`

-> stack should be ".", "."

`del`

## num (1)

`"!wait" num`

-> stack should be 33

`del`

## num (2)

`"" num`

-> stack should be 0

`del`

## size (1)

`"hello" size`

-> stack should be 5

`del`

## size (2)

`"" size`

-> stack should be 0

`del`

## size, str->, ->str

`"hello" str-> ->str size`

-> stack should be 7

`del`

## pos (1)

`"my big string" "big" pos`

-> stack should be 4

`del`

## pos (2)

`"my big string" "bOg" pos`

-> stack should be 0

`del`

## pos (3)

`"my big string" pos`

-> error should be 2

-> stack size should be 1

`del`

## pos (4)

`pos`

-> error should be 2

`del`

## sub (1)

`"my string to sub" 4 6`

`sub`

-> stack should be "str"

`del`

## sub (2)

`"my string to sub" -1 -2 sub`

-> stack should be ""

`del`

## sub (3)

`"my string to sub" 0 0 sub`

-> stack should be ""

`del`

## sub (4)

`"my string to sub" 6 5 sub`

-> stack should be ""

`del`

## sub (5)

`"my string to sub" 100 101 sub`

-> stack should be ""

`del`

## sub (6)

`"my string to sub" 14 100 sub`

-> stack should be "sub"

`del`

## endl 1
`endl type`

-> stack should be "string"

`del`

## endl 2
`endl num`

-> stack should be 10

`del`
