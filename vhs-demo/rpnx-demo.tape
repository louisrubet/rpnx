Output rpnx-demo.gif

Set Shell "bash"
Set FontSize 18
Set Width 900
Set Height 550
Set Padding 20
Set Theme "Dracula"

Type "rpnx"
Enter
Sleep 3s

# Basic RPN arithmetic
Type "Basic RPN arithmetic
Sleep 1s
Ctrl+C
Type "3"
Sleep 1s
Enter
Sleep 1s
Type "4"
Sleep 1s
Enter
Sleep 1s
Type "+"
Sleep 1s
Enter
Sleep 2s
Type "2"
Sleep 0.5s
Enter
Sleep 0.5s
Type "*"
Sleep 1s
Enter
Sleep 3s

Type "3"
Sleep 0.5s
Type " "
Sleep 0.5s
Type "4"
Sleep 0.5s
Type "+"
Sleep 1s
Type " "
Sleep 0.5s
Type "2"
Sleep 0.5s
Type "*"
Sleep 1s
Enter
Sleep 3s
Type "del"
Enter
Sleep 1s

# Arbitrary‑precision
Type "Arbitrary‑precision"
Sleep 1s
Ctrl+C
Type "256 prec pi"
Enter
Sleep 3s
Type "drop"
Enter
Sleep 1s

Type "1024 prec"
Enter
Sleep 1s
Type "100 fact"
Enter
Sleep 3s

Type "del default"
Enter
Sleep 1s

# Complex numbers
Type "Complex numbers"
Sleep 1s
Ctrl+C
Type "(3,4) (1,2) /"
Sleep 2s
Enter
Sleep 2s
Type "(3,4) (1,2) *"
Sleep 2s
Enter
Sleep 2s
Type "(3,4) (1,2) pow"
Sleep 2s
Enter
Sleep 3s
Type "del"
Enter
Sleep 1s

# Trigonometry and logs
Type "Trigonometry and logs"
Sleep 1s
Ctrl+C
Type "pi 4 / sin"
Enter
Sleep 3s

# Bitwise operations
Type "del"
Enter
Sleep 0.5s
Type "Bitwise operations"
Sleep 1s
Ctrl+C
Type "0xCAFE 0b1111111100000000 &"
Enter
Sleep 3s
Type "del"
Enter
Sleep 1s
Type "0xDEAD 0xBEEF ^"
Enter
Sleep 3s

Type "dec"
Enter
Sleep 2s
Type "bin"
Enter
Sleep 2s
Type "hex"
Enter
Sleep 2s

Type "del"
Enter
Sleep 1s

# Global and specific help
Type "Global and specific help"
Sleep 1s
Ctrl+C
Type "help"
Sleep 3s
Enter
Sleep 3s
Type "help del"
Sleep 1s
Enter
Sleep 3s

# Program example: Fahrenheit to Celsius converter
Type "Program: Fahrenheit to Celsius converter"
Sleep 1s
Ctrl+C
Type "212"
Enter
Sleep 1s
Type "<< 32 - 5 9 / * >>"
Enter
Sleep 3s
Type "eval"
Sleep 1s
Enter
Sleep 3s

Type "del"
Enter
Sleep 1s

# Program example: sum of integers from 1 to n
Type "Program: sum of integers from 1 to n"
Sleep 1s
Ctrl+C
Type "<< -> n << 0 1 n for i i + next >> >>"
Enter
Sleep 3s
Type "sum sto"
Enter
Sleep 3s
Type "100 sum"
Enter
Sleep 3s
Type "'sum' rcl"
Enter
Sleep 3s

Type "del"
Enter
Sleep 1s

# Lot more
Type "A lot more on"
Sleep 1s
Ctrl+C
Enter

Type "Arithmetic"
Sleep 1s
Ctrl+C
Enter

Type "Bitwise"
Sleep 1s
Ctrl+C
Enter

Type "Stack operations"
Sleep 1s
Ctrl+C
Enter

Type "Comparisons"
Sleep 1s
Ctrl+C
Enter

Type "Trigonometry, hyperbolics, logs"
Sleep 1s
Ctrl+C
Enter

Type "Variables"
Sleep 1s
Ctrl+C
Enter

Type "Control: if then else"
Sleep 1s
Ctrl+C
Enter

Type "Loops: for..next, while..repeat, do..until"
Sleep 1s
Ctrl+C
Enter

Type "Display: std fix sci prec hex dec bin base"
Sleep 1s
Ctrl+C
Enter

# Exit
Type "q"
Enter
Sleep 1s
