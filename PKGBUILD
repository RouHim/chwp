# Maintainer: Rouven Himmelstein <rouvenhimmelstein@gmail.com>
_gitname=chwp
_cmdname=chwp
pkgname=${_gitname}-git
pkgname=chwp-git
pkgver=0.0.18
pkgrel=1
pkgdesc="Changes the background wallpaper and lockscreen from the command line."
arch=('any')
url="https://github.com/RouHim/chwp"
license=('GPL3')
depends=('xorg-xrandr')
makedepends=('git' 'rust')
provides=('chwp')
conflicts=('chwp')
source=("https://github.com/RouHim/chwp/archive/refs/heads/main.zip")
sha1sums=('SKIP')

build() {
    cargo build --release
}

package() {
    # install executable
    chmod +x target/release/chwp
    cp target/release/chwp /usr/local/bin/

    # install man page
    install -D -m755 chwp.1 ${pkgdir}/usr/share/man/man1/chwp.1
}