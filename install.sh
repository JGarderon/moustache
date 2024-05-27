#!/usr/bin/env bash

# Source : https://github.com/johnthagen/min-sized-rust?tab=readme-ov-file

ARCHI=${ARCHI:-"x86_64-unknown-linux-gnu"}

if ! type cargo &>/dev/null; then
    echo "The Cargo program must be installed on your system to compile."
    echo "See : https://www.rust-lang.org/learn/get-started"
fi

echo "--> COMPILATION step"
RUSTFLAGS="-Zlocation-detail=none" \
	cargo +nightly build \
		-Z build-std=std,panic_abort \
		-Z build-std-features=panic_immediate_abort \
		--target "$ARCHI" \
		--release

echo "--> SIZE REDUCTION step"
if type upx &>/dev/null; then
	upx --best --lzma "target/$ARCHI/release/moustache"
else
	echo "The UPX program must be installed on your system to reduce size."
    echo "See : https://linux.die.net/man/1/upx"
fi

echo "--> TESTS step"
./tests/tests.sh "target/$ARCHI/release/moustache" "./tests/*.test"
if [ $? != 0 ]
then
	exit 1
fi

echo "--> INSTALLATION step"
# sudo cp "target/$ARCHI/release/moustache" /usr/bin/moustache
# sudo chmod 775 /usr/bin/moustache
cp "target/$ARCHI/release/moustache" ~/.local/bin/moustache
echo "Installation in your local bin directory"
