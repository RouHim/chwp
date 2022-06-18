# chwp

Changes the wallpaper from the command line.

## Installation

### Generic

Use the [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) package manager to install
the [chwp](https://crates.io/crates/chwp) crate:

```shell
cargo install chwp
```

### Arch Linux

Build via the [chwp-git](https://aur.archlinux.org/packages/chwp-git) [AUR](https://aur.archlinux.org/):

```shell
yay -S chwp-git
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
chwp https://source.unsplash.com/1920x1080/?autumn
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