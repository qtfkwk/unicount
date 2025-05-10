# Examples

```text
$ unicount -V
unicount 0.1.4
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

