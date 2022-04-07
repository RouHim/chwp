# Maintainer: Rouven Himmelstein <rouvenhimmelstein@gmail.com>

_gitname=chwp
_cmdname=chwp
pkgname=${_gitname}-git
pkgver=0.0.18
pkgrel=1
pkgdesc="Changes the background wallpaper and lockscreen from the command line."
arch=('x86_64')
url="https://github.com/RouHim/chwp"
license=('GPL3')
depends=('xrandr')
makedepends=('git', 'rust')
provides=('chwp')
conflicts=('chwp')
source=("https://github.com/RouHim/chwp/archive/refs/heads/main.zip")
sha1sums=('SKIP')

prepare() {
    # prep build
}

build() {
    cargo build --release
}

package() {
    # install executable
    chmod +x target/release/${_cmdname}
    cp target/release/${_cmdname} /usr/local/bin/

    # install man page
    install -D -m755 ${_cmdname}.1 ${pkgdir}/usr/share/man/man1/${_cmdname}.1
}