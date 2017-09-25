BUILD_TARGET=arm-unknown-linux-gnueabihf

IP_ADDRESS=lampi.local

BUILD_MODE=release

upload:
	make f
	make build
	rsync -vp ./target/${BUILD_TARGET}/${BUILD_MODE}/ledstrips ${IP_ADDRESS}:~/build/ledstrips/ledstrips

build:
	cargo build --target=${BUILD_TARGET} --${BUILD_MODE}

f:
	make -C frontend
	scp ./frontend/output/* ${IP_ADDRESS}:~/build/ledstrips/files
#frontend:
#	make -C frontend
