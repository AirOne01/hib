# Maintainer: Erwann (AirOne01) LAGOUCHE <erwann.lagouche@gmail.com>
pkgname=hib
pkgver=1.0.0
pkgrel=1
pkgdesc="Quick utility for hibernating your system"
arch=(x86_64)
url="https://github.com/AirOne01/hib"
license=('MIT')
groups=()
depends=()
makedepends=(cargo)
optdepends=()
provides=()
conflicts=()
replaces=()
backup=()
options=()
install=
changelog=
source=("git+$url.git")
noextract=()
md5sums=()

prepare() {
  cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --frozen --release --all-features
}

check() {
  export RUSTUP_TOOLCHAIN=stable
  cargo test --frozen --all-features
}

package() {
  install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
}
