# Calculator
A powerful calculator for developers.
It is based on the inbuilt Calculator App on MacOS in "Programmer" Mode.
I use this tool a lot, but it has its limitations.

## TODO

- [ ] from parser create AST then evaluate AST expression.
- [ ] binary, dec and hex output
- [ ] support for binary operators XOR, NOR, AND, OR, << and >>.
- [ ] config file (e.g. always show binary representation);

## Implementation Notes

currently the test cases to test Keywords will fail. The reason for this is, that the current implementation consumes the chars.
Hence `Scanner::new("and");` calls `self.get_keyword()` after reading the last character (d in this case).
get_keyword tries to read the string, but because the chars were consumed, it reads `None` .
