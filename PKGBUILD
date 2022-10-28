# Maintainer: Erwann (AirOne01) LAGOUCHE <erwann.lagouche@gmail.com>
pkgname=hibe
pkgver=1.0.0
pkgrel=1
pkgdesc="Quick utility for hibernating your system"
arch=(x86_64)
url="https://github.com/AirOne01/hibe"
license=('MIT')
groups=()
depends=('libgcc_s>=1-64' 'libc>=6-64' 'ld-linux-x86-64>=2-64' 'systemd>=251.7-1')
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
sha256sums=('SKIP')

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
