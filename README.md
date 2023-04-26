# Split First Char

A small utility to split a string into the first character (type `char`) and the rest (type `&str`).

## Usage

### Function call

```rust
use split_first_char::split_first_char;
let (first_char, rest) = split_first_char("abc").unwrap();
assert_eq!(first_char, 'a');
assert_eq!(rest, "bc");
```

### Method call

```rust
use split_first_char::SplitFirstChar;
let (first_char, rest) = "abc".split_first_char().unwrap();
assert_eq!(first_char, 'a');
assert_eq!(rest, "bc");
```

## Alternative

If you don't need the first character to be a `char`, just use `str.split_at(1)`, it will return a tuple of 2 strings.

## License

[MIT](https://github.com/KSXGitHub/split-first-char/blob/master/LICENSE.md) © [Hoàng Văn Khải](https://ksxgithub.github.io/).
