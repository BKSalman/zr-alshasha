# THIS APP IS STILL WORK IN PROGRESS

# Zr Alshasha زر الشاشة

Zr alshaha is a simple app that displays your keyboard input written in Rust!

it's useful for diplaying your keyboard input while streaming, making tutorials, and such

# Small Demo

[zr-alshasha.webm](https://user-images.githubusercontent.com/85521119/197903583-5bec6f96-b92b-471f-9650-0d6eeaaeb58c.webm)


# Key binds

| Key               | Description     |
| ----------------- | --------------- |
| Right-Click(hold) | Move the window |

# Features

- Show pressed keys 

- Config file

    for now, you can specify the window position and the font size using a toml file in your config directory under "zr-alshasha"

    Linux:   /home/alice/.config/zr-alshasha/config.toml
    
    Windows: C:\Users\Alice\AppData\Roaming\zr-alshasha\config.toml
    
    Mac:     /Users/Alice/Library/Application Support/zr-alshasha/config.toml

    ```toml
    font_size = 30 # Default is 30

    width = 1000 # Default is 500

    erase_on_backspace = true # Default is false

    [position]
    x = 2000 # Default is 1000
    y = 2000 # Default is 1000

    ```


You can track features in [this issue](https://github.com/BKSalman/zr-alshasha/issues/4)

# Installation
## Linux

### Fedora
download the rpm package from the releases page then install it with ``sudo dnf localinstall <rpm_package_name>``

### Ubuntu
download the deb package from the releases page then install it with ``sudo apt install ./<deb_package_name>``

### other distributions

download the ``zr-alshasha`` binary from the [realeases page](https://github.com/BKSalman/ytdlp-gui/releases)

## Windows
##### just download the zip file from the releases page, extract it in a subfolder and start the ``zr-alshasha.exe``


## what does Zr Alshasha mean

it just means screen key in Arabic
