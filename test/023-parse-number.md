# PARSE NUMBER

`default del`

## type

`1.0 type`

-> stack should be "number"

`del`

## 1-shot entries

```
0
0.
.0
0.0
-0
+0
+.0
+0.
0.1
+0.1
-0.1
-.1
+.1
```

-> stack should be 0, 0, 0, 0, 0, 0, 0, 0, 0.1, 0.1, -0.1, -0.1, 0.1

`del`

## n-shot entries

```
0 0. .0 0.0 -0 +0 +.0 +0. 0.1 +0.1 -0.1 -.1 +.1
```

-> stack should be 0, 0, 0, 0, 0, 0, 0, 0, 0.1, 0.1, -0.1, -0.1, 0.1

`del`

## numb 1

`3.14 +3.14 -3.14`

-> stack should be 3.14, 3.14, -3.14

`del`

## spaces

` -3.14 -3 .14`

-> stack should be -3.14, -3, 0.14

`del`

## exp entry

`+3.14e2`

-> stack should be 314

`del`

## inf nan

`+inf inf -inf nan`

-> stack should be inf, inf, -inf, nan

`del`

