# ERROR COMMANDS

`default del`

## error - no error (0)

`1 2 + drop`

`error strerror`

-> stack should be 0, "ok"

`del`

## error - missing operand (2)

`+`

`error strerror`

-> stack should be 2, "missing operand"

`del`

## error - bad operand type (3)

`"hello" sin`

`error strerror`

-> stack should be "hello", 3, "bad operand type"

`del`

## error - unknown variable (5)

`'unknown_var_xyz' rcl`

`error strerror`

-> stack should be 'unknown_var_xyz', 5, "unknown variable"

`del`

## error/strerror preserve last error

`+`

`error`

-> stack should be 2

`error`

-> stack should be 2, 2

`strerror`

-> stack should be 2, 2, "missing operand"

`del`

## error cleared after success

`+`

`error`

-> stack should be 2

`del`

`1 2 +`

`error strerror`

-> stack should be 3, 0, "ok"

`del`
