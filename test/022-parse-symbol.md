# PARSE SYMBOL

`default del `

## type

`'hey' type`

-> stack should be 'symbol'

`del`

## void 1

`''`

-> stack should be ''

`del`

## void 2

`'`

-> stack should be ''

`del`

## symbol

`'abcd'`

-> stack should be 'abcd'

`del`

## unterminated

`'abcd`

-> stack should be 'abcd'

`del`

## unterminated 2

`'abcd' 'abc`

-> stack should be 'abcd', 'abc'

`del`

## spaces

`'abc d'`

-> stack should be 'abc d'

`del`

## spaces 2

`'   abcd '`

-> stack should be '   abcd '

`del`

## spaces 3

`'   abcd ' 'def' 'gh ij'`

-> stack should be '   abcd ', 'def', 'gh ij'

`del`

## spaces 4

`' . abcd . ;;  '`

-> stack should be ' . abcd . ;;  '

`del`

## spaces 5

`' . abcd . ;;  ' 'ab  c`

-> stack should be ' . abcd . ;;  ', 'ab  c'

`del`

## nested types 1

`'1'`

-> stack should be '1'

`del`

## nested types 2

`'1.0 swap drop`

-> stack should be '1.0 swap drop'

`del`
