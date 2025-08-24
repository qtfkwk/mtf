# Markdown Table Formatter

## CLI

```text
$ cat table.md
!run:cat ../table.md
$ mtf table.md
!run:../target/release/mtf ../table.md
```

## Text Editor

1. Use a text editor (Vim, NeoVim, Helix Editor, ...)
2. Select Markdown table content in a file
   (in Helix Editor type `␛x` then `x` repeatedly, or type `␛v` then hjkl or arrows)
3. Pass it as input to `mtf`
   (in Helix Editor type `|mtf⏎`)
4. `mtf` formats the table content

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

