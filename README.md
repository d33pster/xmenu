# xmenu

Interactive terminal menu lib for rust.

## Table of Contents

- [Overview](#overview)
- [Installation](#installation)
- [Usage](#usage)
- [Demo](#demo)
- [Fix Windows CMD ANSI Colour Problem](#fix-windows-cmd-ansi-colour-problem)

## Overview

make interactive menus in rust. In case of any issues feel free to create an issue [here](https://github.com/d33pster/xmenu/issues) or create a pull request.

## Installation

```bash
cargo install xmenu
```

This will also install a bin named `xmenu` which can be called for simulating demos of xmenu working as well as fix an ISSUE in Windows terminal where ANSI colours are not displaying as it is supposed to. Instead, the colour codes are getting printed.

Just choose the option for fixing it after running `xmenu` in the terminal

```bash
xmenu
```

## Usage

For in script usage,

```rust
use xmenu::{Xmenu, Colour};

fn main() {
    // define the menu
    let mut xm = Xmenu::new();
    
    // add options
    xm.add("Option1");
    xm.add("Option2");

    // run and collect result
    let result = xm.run(Colour::Blue); // this will make the selected option blue

    // create conditions
    if result == "Option1".to_string() {
        // do something
    } else if result == "Option2".to_string() {
        // do something else
    } else result == "Abort" {
        std::process::exit(0);
    }
}
```

## Demo

For a demo, after installing `xmenu` using `cargo install xmenu`, run:

```bash
xmenu
```

in terminal.

## Fix Windows CMD ANSI Colour Problem

After running `xmenu` in terminal, choose the third option.
