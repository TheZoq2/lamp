BUILD_TARGET=arm-unknown-linux-gnueabihf

IP_ADDRESS=192.168.0.104

upload:
	make build
	make f
	scp ./target/${BUILD_TARGET}/debug/ledstrips ${IP_ADDRESS}:~/build/ledstrips/ledstrips

build:
	cargo build --target=${BUILD_TARGET}

f:
	make -C frontend
	scp ./frontend/output/* ${IP_ADDRESS}:~/build/ledstrips/files
#frontend:
#	make -C frontend
