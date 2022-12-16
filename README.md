# RustRecursion

Let's consider strings which can contain only latin alphabet letters a .. z.
Let’s define string comparison in the following way:
- if string lengths differ: strings are compared by length (a greater string has a greater length)
- else: strings are compared lexicographically

Please write a program which takes two strings and prints all strings which are greater than the first one (and have the same length) and smaller than the second one (and have the same length).

Example input 1:
```
zx aac
```

Example output 1:
```
zy
zz
aaa
aab
```

Example input 2:
```
fzx gac
```

Example output 2:

```
fzy
fzz
…
zzz
aaa
…
gaa
gab
```

### Run
```
cargo run
```

### Test
```
cargo test
```