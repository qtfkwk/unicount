# Examples

```rust
assert_eq!(
    unicount_lib::Kind::EnglishLower.counter(0).take(30).collect::<Vec<_>>(),
    vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
        "r", "s", "t", "u", "v", "w", "x", "y", "z", "aa", "ab", "ac", "ad"
    ],
);

assert_eq!(
    unicount_lib::Kind::EnglishUpper.counter(0).take(30).collect::<Vec<_>>(),
    vec![
      "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q",
      "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "AA", "AB", "AC", "AD"
    ],
);

assert_eq!(
    unicount_lib::Counter::new("abc", 0).take(30).collect::<Vec<_>>(),
    vec![
        "a", "b", "c", "aa", "ab", "ac", "ba", "bb", "bc", "ca", "cb", "cc", "aaa", "aab",
        "aac", "aba", "abb", "abc", "aca", "acb", "acc", "baa", "bab", "bac", "bba", "bbb",
        "bbc", "bca", "bcb", "bcc"
    ],
);
```

