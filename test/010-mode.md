# MODE
`default del`

## std (1)

```
del
38 std
```

-> stack size should be 0

-> error should be 0

`del`

## std (2)

`1 3 /`

-> stack should be 0.33333333333333333333333333333333333333

`del`

## fix (1)

`10 fix 1`

-> stack should be 1.0000000000

`del`

## fix (2)

`1 4 fix`

-> stack should be 1.0000

`del`

## fix out of range

`-1 fix`

-> error should be 4

`del`

## table for fixed entry, 4 fix output >= 0

```
4 fix
1 0.01 0.0001 0.00006 0.00004 0 0.012 0.001256 100 100.001 100.00006 100.00004 12345678910111213.12355
```

-> stack should be 1.0000, 0.0100, 0.0001, 0.0001, 0.0000, 0.0000, 0.0120, 0.0013, 100.0000, 100.0010, 100.0001, 100.0000, 12345678910111213.1236

`del default`

## table for fixed entry, 4 fix output <= 0

```
4 fix
-1 -0.01 -0.0001 -0.00006 -0.00004 -0 -0.012 -0.001256 -100 -100.001 -100.00006 -100.00004 -12345678910111213.12355
```

-> stack should be -1.0000, -0.0100, -0.0001, -0.0001, -0.0000, -0.0000, -0.0120, -0.0013, -100.0000, -100.0010, -100.0001, -100.0000, -12345678910111213.1236

`del default`

## sci (1)

`1 12 sci`

-> stack should be 1.000000000000e+00

`del`

## sci (2)

`1 2 sci`

-> stack should be 1.00e+00

## sci out of range

`-1 sci`

-> error should be 4

`del`

## table for sci entry, 4 fix output >= 0

```
4 fix
0e100 1e0 1.e0 1.001e0 1.e-3 1.e-4 6.e-5 4.e-5 1.00001e2 1.0000006e2 1.0000004e2 1234.5678917e2
```

-> stack should be 0.0000, 1.0000, 1.0000, 1.0010, 0.0010, 0.0001, 0.0001, 0.0000, 100.0010, 100.0001, 100.0000, 123456.7892

`del default`

## table for sci entry, 4 fix output <= 0

```
4 fix
-0e100 -1e0 -1.e0 -1.001e0 -1.e-3 -1.e-4 -6.e-5 -4.e-5 -1.00001e2 -1.0000006e2 -1.0000004e2 -1234.5678917e2
```

-> stack should be -0.0000, -1.0000, -1.0000, -1.0010, -0.0010, -0.0001, -0.0001, -0.0000, -100.0010, -100.0001, -100.0000, -123456.7892

`del default`

## default
`1 4 fix`

-> stack should be 1.0000

`default`

-> stack should be 1

`del`

## table for singularity

```
4 fix
nan @nan@ -nan inf -inf @inf@ -@inf@
```

-> stack should be nan, nan, nan, inf, -inf, inf, -inf

`del default`

## some strange behaviour (1)

```
0 fix
1 2 / dup +
```

-> stack should be 1

`del default`

## some strange behaviour (2)

```
1 fix
0.6
```

-> stack should be 0.6

`del default`

## some strange behaviour (3)

```
0 fix
110.6 0.6
```

-> stack should be 111, 1

`del default`
