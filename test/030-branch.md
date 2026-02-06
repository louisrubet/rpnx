# BRANCH

`default del`

## if then else end (1)
`1 if then 'ok' end`

-> stack should be 'ok'

`del`

## if then else end (2)

`1 if 'before' then 'ok' end`

-> stack should be 'before', 'ok'

`del`

## if then else end (3)

`0 if then 'ok' end`

-> stack size should be 0

`del`

## if then else end (4)

`0 if then 'ok' end`

-> stack size should be 0

`del`

## if then else end (5)

`1 if then 'ok' else 'KO' end`

-> stack should be 'ok'

`del`

## if then else end (6)

`1 if then 'ok' 'dokey' else 'KO' end`

-> stack should be 'ok', 'dokey'

`del`

## if then else end (7)

`0 if then 'ok' else 'KO' end`

-> stack should be 'KO'

`del`

## if then else end - error case (1)

`if then end`

-> error should be 2

`del`

## if then else end - error case (2)

`0 if then`

-> error should be 11

`del`

## if then else end - error case (3)

`0 if end`

-> error should be 11

`del`

## if then else end - error case (4)

`0 if end`

-> error should be 11

`del`

## if then else end - error case (5)

`then`

-> error should be 11

`del`

## if then else end - error case (6)

`1 if`

-> error should be 11

`del`

## if then else end - error case (7)

`else`

-> error should be 11

`del`


## if then else end - error case (8)

`end`

-> error should be 11

`del`

## if then else end - error case (9)

`"1" if then end`

-> error should be 3

`del`

## ift (1)

`1 'ok' ift`

-> stack should be 'ok'

`del`

## ift (2)

`0 'ok' ift`

-> stack size should be 0

`del`

## ift (3)

`'ok' ift`

-> error should be 2

-> stack size should be 1

`del`

## ift (4)

`ift`

-> error should be 2

-> stack size should be 0

`del`

## ifte (1)

`1 'ok' 'nok' ifte`

-> stack should be 'ok'

-> stack size should be 1

`del`

## ifte (2)

`0 'ok' 'nok' ifte`

-> stack should be 'nok'

-> stack size should be 1

`del`

## ifte (3)

`'ok' 'nok' ifte`

-> error should be 2

-> stack size should be 2

`del`

## ifte (4)

`'nok' ifte`

-> error should be 2

-> stack size should be 1

`del`

## ifte (5)

`ifte`

-> error should be 2

-> stack size should be 0

`del`
## start next (1)

`1 2 start 0 next`

-> stack should be 0, 0

`del`

## start next (2)

`2 1 start 0 next`

-> stack size should be 0

`del`

## start next (3)

`-2 -1 start 0 next`

-> stack should be 0, 0

`del`

## start next (4)

`-1 -2 start 0 next`

-> stack size should be 0

`del`

## start next (5)

`1 1 start 0 next`

-> stack should be 0

`del`

## start next - cloning objects (1)

`1 2 start 'ok' next`

-> stack should be 'ok', 'ok'

`del`

## start next - cloning objects (2)

`1 2 start ok next`

-> stack should be 'ok', 'ok'

`del`

## start next - cloning objects (3)

`1 2 start "ok" next`

-> stack should be "ok", "ok"

`del`

## start next - cloning objects (4)

`1 2 start (1,2) next`

-> stack should be (1,2), (1,2)

`del`

## start next - cloning objects (5)

`1 2 start «ok» next`

-> stack should be « ok », « ok »

`del`

## start next - error case (1)

`1 start next`

-> error should be 2

`del`

## start next - error case (2)

`start next`

-> error should be 2

`del`

## start next - error case (3)

`start`

-> error should be 11

`del`

## start next - error case (4)

`next`

-> error should be 11

`del`

## start next - error case (5)

`"1" 2 start next`

-> error should be 3

`del`

## start next - error case (6)

`1 "2" start next`

-> error should be 3

`del`

## for next (1)

`23 27 for i i next`

-> stack should be 23, 24, 25, 26, 27

`del`

## for next (2)

`1 1 for i i next`

-> stack should be 1

`del`

## for next (3)

`27 23 for i i next`

-> stack size should be 0

`del`

## for next (4)

`-2 -1 for i i next`

-> stack should be -2, -1

`del`

## for next (5)

`-1 -2 for i i next`

-> stack size should be 0

`del`

## for next - loop variable overwrite

`123 'i' sto 1 2 for i i next`

-> stack should be 1, 2

`del`

## nested for next

`1 2 for i 0 1 for j i (1,0) * j (0,1) * + next next`

-> stack should be (1,0), (1,1), (2,0), (2,1)

`del`

## for next - error case (1)

`1 for i i next`

-> error should be 2

`del`

## for next - error case (2)

`for i i next`

-> error should be 2

`del`

## for next - error case (3)

`"1" 2 for i i next`

-> error should be 3

`del`

## for next - error case (4)

`1 "2" for i i next`

-> error should be 3

`del`

## for next - error case (5)

`1 2 for i i`

-> error should be 11

`del`

## for next - error case (6)

`for`

-> error should be 11

`del`

## for step (1)

`23 27 for i i 1 step`

-> stack should be 23, 24, 25, 26, 27

`del`

## for step (2)

`0 1 for i i 0.25 step`

-> stack should be 0, 0.25, 0.5, 0.75, 1

`del`

## for step (3)

`-1 0 for i i 0.25 step`

-> stack should be -1, -0.75, -0.5, -0.25, 0

`del`

## for step (4)

`0 -1 for i i 0.25 step`

-> stack size should be 0

`del`

## for step (5)

`0 -1 for i i -0.25 step`

-> stack size should be 0

`del`

## for step (6) - check boundary integration

`1 2 for i i 0.2 step`

-> stack should be 1, 1.2, 1.4, 1.6, 1.8, 2

`del`

## for step (7) - check boundary integration

`1 2 for i i 0.5 step`

-> stack should be 1, 1.5, 2

`del`

## nested for step

`0 2 for i 0 6 for j i (1,0) * j (0,1) * + 3 step 2 step`

-> stack should be (0,0), (0,3), (0,6), (2,0), (2,3), (2,6)

`del`

## for step - error case (1)

`0 1 for i i "0.5" step`

-> error should be 3

`del`

## for step - error case (2)

`step`

-> error should be 11

`del`

## do..until (1)

`do 'ok' until 1 end`

-> stack should be 'ok'

`del`

## do..until (2)

`do until 1 end`

-> stack size should be 0

`del`

## do..until (3)

`3 do 1 - 'ok' swap dup until 0 == end drop`

-> stack should be 'ok', 'ok', 'ok'

`del`

## do..until (4)

`1 'a' sto do a 1 + 'a' sto until a 3 > end a`

-> stack should be 4

`del`

## nested do..until

`1 'i' sto do 0 'j' sto do i (1,0) * j (0,1) * + 1 'j' sto+ until j 1 > end 1 'i' sto+ until i 2 > end`

-> stack should be (1,0), (1,1), (2,0), (2,1)

`del`

## do..until error case (1)

`do`

-> error should be 11

`del`

## do..until error case (2)

`do 8 end`

-> error should be 11

`del`

## do..until error case (3)

`until`

-> error should be 11

`del`

## do..until error case (4)

`do 3 until`

-> error should be 11

`del`

## do..until error case (5)

`until 1 end`

-> error should be 11

`del`

## do..until error case (6)

`do 3 repeat 8 end`

-> error should be 11

`del`

## do..until error case (7)

`do 3 until 8 until 9 end`

-> error should be 11

`del`

## while..repeat (1)

`while 0 repeat ok end`

-> stack size should be 0

`del`

## while..repeat (2)

`2 while dup 0.1 > repeat dup 2 / end`

-> stack should be 2, 1, 0.5, 0.25, 0.125, 0.0625

`del`

## while..repeat (3)

`0 'a' sto while a 3 < repeat a 1 + 'a' sto 100 0 a for b b + next end`

-> stack should be 101, 103, 106

`del`

## nested while .. repeat

`1 'i' sto while i 2 <= repeat 0 'j' sto while j 1 <= repeat i (1,0) * j (0,1) * + 1 'j' sto+ end 1 'i' sto+ end`

-> stack should be (1,0), (1,1), (2,0), (2,1)

`del`

## while..repeat error case (1)

`while`

-> error should be 11

`del`

## while..repeat error case (2)

`while 3 end`

-> error should be 11

`del`

## while..repeat error case (3)

`repeat`

-> error should be 11

`del`

## while..repeat error case (4)

`while 1 repeat`

-> error should be 11

`del`

## while..repeat error case (5)

`repeat 1 end`

-> error should be 11

`del`

## while..repeat error case (6)

`while 3 repeat 8 repeat 9 end`

-> error should be 11

`del`

## while..repeat error case (7)

`while 3 until 8 end`

-> error should be 11

`del`
