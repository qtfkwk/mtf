# Markdown Table Formatter

## CLI

```text
$ cat table.md
!run:cat ../table.md
$ mtf table.md
!run:../target/release/mtf ../table.md
```

## Library

```rust
let input = "\\
w | x | y | z
---|:---|---:|:---:
1 | 1 | 1 | 1
2 | 4 | 16 | 256
3 | 9 | 81 | 6561
4 | 16 | 256 | 65536

";

let output = "\\
| w  | x  |   y |   z   |
|----|:---|----:|:-----:|
| 1  | 1  |   1 |   1   |
| 2  | 4  |  16 |  256  |
| 3  | 9  |  81 | 6561  |
| 4  | 16 | 256 | 65536 |

";

assert_eq!(mtf::process(input).unwrap(), output);
```

