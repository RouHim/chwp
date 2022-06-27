# chwp

Changes the wallpaper from the command line.

## Installation

### Build from source

Use the [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) package manager to install
the [chwp](https://crates.io/crates/chwp) crate:

```shell
cargo install chwp
```

### Download latest binary

This snippet will download and install the latest chwp release:

```shell
LATEST_VERSION=$(curl -L -s -H 'Accept: application/json' https://github.com/RouHim/chwp/releases/latest | sed -e 's/.*"tag_name":"\([^"]*\)".*/\1/') && \
sudo curl -L -o /usr/bin/chwp https://github.com/RouHim/chwp/releases/download/$LATEST_VERSION/chwp-linux-x86_64 && \
sudo chmod +x /usr/bin/chwp
```

### Arch Linux

Install [chwp-bin](https://aur.archlinux.org/packages/chwp-bin) [AUR](https://aur.archlinux.org/) package:

```shell
yay -S chwp-bin
```

## Usage

```shell
chwp [keywords]|[imageurl] span
```

### Examples

Set a random wallpaper for the keyword ocean:

```shell
chwp ocean
```

Set a wallpaper from a specific image url:

```shell
chwp https://source.unsplash.com/1920x1080
```

Set a wallpaper from multiple keywords, a random one will be selected:

```shell
chwp water,sky,ocean
```

Combine multiple keywords with a `+`:

```shell
chwp night+city
```

The `span` parameter can be used to span the wallpaper over multiple screens:

```shell
chwp ocean span
```