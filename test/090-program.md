# PROGRAM

`default del`

## program

`<< 'one' >>`

`« 'one' »`

-> stack size should be 2

-> stack should be « 'one' », « 'one' »

`del`

## program (2)

`<< 'one' 2`

`« 'one' 2`

-> stack size should be 2

-> stack should be « 'one' 2 », « 'one' 2 »

`del`

## program (3)

`<<`

`«`

-> stack size should be 2

-> stack should be «  », «  »

`del`

## program (4)

`<< << << <<`

`« « « «`

-> stack size should be 2

-> stack should be « « « «  » » » », « « « «  » » » »

`del`

## program (5)

`<< -> n << n 2 * >> >>`

`« -> n « n 2 * » »`

-> stack size should be 2

-> stack should be « -> n « n 2 * » », « -> n « n 2 * » »

`del`

## spaces in program entry

`<<->n<<1`
`<< ->n<<1`
`<<-> n<<1`
`<<->n <<1`
`<<->n<< 1`
`<< ->n<<1`
`<< -> n<<1`
`<< ->n <<1`
`<< ->n<< 1`
`<<-> n <<1`
`<<-> n<< 1`
`<<->n << 1`

-> stack should be « -> n « 1 » », « -> n « 1 » », « -> n « 1 » », « -> n « 1 » », « -> n « 1 » », « -> n « 1 » », « -> n « 1 » », « -> n « 1 » », « -> n « 1 » », « -> n « 1 » », « -> n « 1 » », « -> n « 1 » »

`del`

## program imbrication

`<< 1 << 2 >> >>`

-> stack should be « 1 « 2 » »

`del`

## program evaluation

`<< 1 << 2 >> >> dup eval`

-> stack should be « 1 « 2 » », 1, « 2 »

`del`

## program 1 arg

`default`

`10`

`<< -> n << 0 1 n for i i 2 * inv + next >> >>`

`eval`

-> stack should be 1.4644841269841269841269841269841269841

`del`

## nested programs

`0 1 10 << -> u0 u1 n << u0 u1 1 n start dup2 + rot drop next swap drop >> >> eval`

`eval`

-> stack should be 89

`del`

## fibo

```
«dup 1 > if then dup 1 - fibo swap 2 - fibo + else 1 == 1 0 ifte end» 'fibo' sto
7 fibo
13 == if then 'ok!' end
```

-> stack should be 'ok!'

`del`

## nested programs with local variables

```
100
10
«dup2 * -> a b c << a b + c / -> d << << -> sym << sym ->str " is " + sym rcl ->str + >> >> 'stringify' sto 'a' stringify 'b' stringify 'c' stringify 'd' stringify >> >>»
eval
```

-> stack should be "'a' is 100", "'b' is 10", "'c' is 1000", "'d' is 0.11"

`del`

## local variables multiple entries

`1 2 3 << -> a b c << "a is " a ->str + "b is " b ->str + "c is " c ->str + >> >> eval`

-> stack should be "a is 1", "b is 2", "c is 3"

`del`

## local variables separation (1)

`123 'n' sto 1 << -> n << "n is " n ->str + >> >> eval`

-> stack should be "n is 1"

`del`

## local variables separation (2)

`123 'n' sto 2 << -> n << n sq << -> n << "n is " n ->str + >> >> >> eval >> eval`

-> stack should be "n is 4"

`del`

## local variables separation (3)

`123 'n' sto 2 << -> n << n sq << -> n << "n is " n ->str + >> >> >> eval >> eval n`

-> stack should be "n is 4", 123

`del`
