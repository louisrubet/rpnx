# PARSE PROGRAM

`default del `

## type

`<< I >> type`

-> stack should be 'program'

`del`

## prog 1

`<< I am a program >>`

-> stack should be « I am a program »

`del`

## prog 2

`<<I am a program>>`

-> stack should be « I am a program »

`del`

## prog 3

`<< I am a program >>`

-> stack should be « I am a program »

`del`

## prog 4

`«I am a program»`

-> stack should be « I am a program »

`del`

## prog 5

`« I am a program  »`

-> stack should be « I am a program »

`del`

## unterminated 1

`<< prog`

-> stack should be « prog »

`del`

## unterminated 2

`« prog`

-> stack should be « prog »

`del`

## unterminated 3

`<< prog>`

-> stack should be « prog> »

`del`

## unterminated 4

`<<`

`«`

-> stack should be «  », «  »

`del`
