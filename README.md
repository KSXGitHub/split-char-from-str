# Split Char From Str

A small utility to split a string into the first or last character (type `char`) and the rest (type `&str`)

## Usage

### Function call

```rust
use split_char_from_str::split_first_char;
let (head, tail) = split_first_char("abc").unwrap();
assert_eq!(head, 'a');
assert_eq!(tail, "bc");
```

```rust
use split_char_from_str::split_last_char;
let (init, last) = split_last_char("abc").unwrap();
assert_eq!(init, "ab");
assert_eq!(last, 'c');
```

### Method call

```rust
use split_char_from_str::SplitCharFromStr;
let (head, tail) = "abc".split_first_char().unwrap();
assert_eq!(head, 'a');
assert_eq!(tail, "bc");
```

```rust
use split_char_from_str::SplitCharFromStr;
let (init, last) = "abc".split_last_char().unwrap();
assert_eq!(init, "ab");
assert_eq!(last, 'c');
```

## Alternative

If you don't need the first character to be a `char`, just use `str.split_at(1)`, it will return a tuple of 2 strings.

## License

[MIT](https://github.com/KSXGitHub/split-first-char/blob/master/LICENSE.md) © [Hoàng Văn Khải](https://github.com/KSXGitHub/).
