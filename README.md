# About

This repository hosts the [`unicount-lib`] library crate and the [`unicount`] command line utility crate.

[`unicount`]: https://crates.io/crates/unicount
[`unicount-lib`]: https://crates.io/crates/unicount-lib

## Library

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


## Command line utility

```text
$ unicount -V
unicount 0.1.2
```

```text
$ unicount -h
Alphabetic counter supporting unicode

Usage: unicount [OPTIONS]

Options:
  -k, --kind <KIND>            Kind [default: english-upper] [possible values:
                               english-upper, english-lower]
  -s, --start <START>          Start [default: 0]
  -t, --take <TAKE>            Take [default: 100]
  -S, --separator <SEPARATOR>  Separator [default: \n]
  -a, --alphabet <ALPHABET>    Custom alphabet
  -h, --help                   Print help
  -V, --version                Print version
```

```text
$ unicount
A
B
C
...
CT
CU
CV
```

```text
$ unicount -k english-lower
a
b
c
...
ct
cu
cv
```

```text
$ unicount -s 23 -t 6
X
Y
Z
AA
AB
AC

```

```text
$ unicount -a abc -t 10
a
b
c
aa
ab
ac
ba
bb
bc
ca
```

```text
$ unicount -S ", " -t 10
A, B, C, D, E, F, G, H, I, J
```


