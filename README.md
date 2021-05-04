# pecho

Short for Painted Echo, an alternative utility to `echo` with easy flags for coloring output.

## Usage

```
FLAGS:
    -k, --black
    -K, --black-bg
    -b, --blue
    -B, --blue-bg
    -l, --bright       Use the bright variant
    -L, --bright-bg    Use the bright background variant
    -q, --cyan
    -Q, --cyan-bg
    -g, --green
    -G, --green-bg
    -h, --help         Prints help information
    -E                 Treat backslashes literally
    -n                 No newline at the end
    -p, --purple
    -P, --purple-bg
    -r, --red
    -R, --red-bg
    -V, --version      Prints version information
    -w, --white
    -W, --white-bg
    -y, --yellow
    -Y, --yellow-bg

OPTIONS:
    -c, --color <color>         Specify color using an argument. Overrides single color options
    -C, --color-bg <colorBg>    Specify background color using an argument. Overrides single color options
    -s, --style <style>...      Styling
    -t, --truecolor <hex>       Hex color in xxxxxx format. Overrides other color options
    -T, --truecolor-bg <hex>    Background in hex in xxxxxx format. Overrides other color options
```
