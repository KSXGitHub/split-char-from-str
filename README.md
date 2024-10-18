# Split Char From Str

A small utility to split a string into the first or last character (type `char`) and the rest (type `&str`)

## Usage

### Function call

```rust
use split_char_from_str::split_first_char;
let (first_char, rest) = split_first_char("abc").unwrap();
assert_eq!(first_char, 'a');
assert_eq!(rest, "bc");
```

```rust
use split_char_from_str::split_last_char;
let (rest, last_char) = split_last_char("abc").unwrap();
assert_eq!(rest, "ab");
assert_eq!(last_char, 'c');
```

### Method call

```rust
use split_char_from_str::SplitCharFromStr;
let (first_char, rest) = "abc".split_first_char().unwrap();
assert_eq!(first_char, 'a');
assert_eq!(rest, "bc");
```

```rust
use split_char_from_str::SplitCharFromStr;
let (rest, last_char) = "abc".split_last_char().unwrap();
assert_eq!(rest, "ab");
assert_eq!(last_char, 'c');
```

## Alternative

If you don't need the first character to be a `char`, just use `str.split_at(1)`, it will return a tuple of 2 strings.

## License

[MIT](https://github.com/KSXGitHub/split-char-from-str/blob/master/LICENSE.md) © [Hoàng Văn Khải](https://github.com/KSXGitHub/).
