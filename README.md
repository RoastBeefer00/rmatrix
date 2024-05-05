# rmatrix

rmatrix is an implementation of the [cmatrix](https://github.com/abishekvashok/cmatrix) project, but in Rust using the [ratatui crate](https://github.com/ratatui-org/ratatui).
![rmatrix](./docs/rmatrix.gif)

## Installation
Someone already had the name "rmatrix", so I had to call it "rjmatrix".
```bash
cargo install rjmatrix
```

## Features
### Resize
Properly handles resizing of the terminal window both vertically and horizontally.

### Colors
Currently supports the following colors:
- red
- green
- yellow
- blue
- purple
- cyan
- rainbow

Cycle through random colors by pressing `c`.

### Speed
Change the speed the matrix falls on a scale of 1-10 (1 being the slowest and 10 being the fastest).

Cycle through the speed by pressing `0-9` (0 being the equivalent of 10).

## Help
```
Usage: rmatrix [OPTIONS]

Options:
-c, --color <COLOR>  Available colors: blue, cyan, red, purple, yellow, green, rainbow
-s, --speed <SPEED>  Speed: 1-10
-h, --help           Print help

```

## Future Improvements
- Add more colors
