# Calculator
A powerful calculator for developers.
It is based on the inbuilt Calculator App on MacOS in "Programmer" Mode.
I use this tool a lot, but it has its limitations.

## TODO

- [x] from parser create AST then evaluate AST expression.
- [ ] add better error handling. Add pre and postconditions!
- [x] binary, dec and hex output
- [x] support for binary operators XOR, NOR, AND, OR, << and >>.
- [ ] config file (e.g. always show binary representation);

## Overview
Currently the following operations are supported:
- the basic 4 arithmetic operations
- << and >> for shift left/right
- &/and/AND for binary and
- |/or/OR for binary or
- ^/xor/XOR for binary xor
- nor/NOR for binary nor
- mod/MOD/% for modulo
- ! for negation (1's complement)
- ~ for 2's complement
- parens for nested expressions

## Commands
The calculator supports a few convenience commands.
- exit/quit to quit the application (alternative press CTRL-C)
- copy bin/dec/hex copies the result in the specified format to the systems clipboard

## Example
![demo](https://j.gifs.com/99mDpZ.gif)
