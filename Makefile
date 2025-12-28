all:
	cargo build --release
release: 
	cargo build --release 
	mkdir -p build 
	cp target/release/typecmd build/
	mv build TypeCmd 
	tar cavf TypeCmd$(VERSION).tar.xz TypeCmd 
	rm -rf TypeCmd 
	mkdir -p release 
	mv TypeCmd$(VERSION).tar.xz release 
install:
	cp target/release/typecmd /usr/local/bin
