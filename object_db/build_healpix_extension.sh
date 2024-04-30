apt-get update -y
apt-get install -y curl git make gcc postgresql-server-dev-16

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
export PATH="$HOME/.cargo/bin:$PATH"

cd /tmp
git clone https://github.com/cds-astro/cds-healpix-rust
cd cds-healpix-rust/libpsql

RUSTFLAGS='-C target-cpu=native' cargo build --release
make
make install
