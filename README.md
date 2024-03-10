# rmatrix

rmatrix is an implementation of the [cmatrix](https://github.com/abishekvashok/cmatrix) project, but in Rust using the [ratatui crate](https://github.com/ratatui-org/ratatui).
![rmatrix](./docs/rmatrix.gif)

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

### Static
Change the speed the matrix falls on a scale of 1-10 (1 being the slowest and 10 being the fastest).

## Help
```
Usage: rmatrix [OPTIONS]

Options:
-c, --color <COLOR>  Available colors: blue, cyan, red, purple, yellow, green
-s, --speed <SPEED>  Speed: 1-10
-h, --help           Print help

```

## Future Improvements
- Shrink matrix down to size of terminal window when size is decreased
    > Currently only grows because a size increase will cause panic if not handled but decrease does not
- Add more colors
- Dynamically change colors and speed while program is running
    - `0-9` to change speed
    - `c` to cycle colors


