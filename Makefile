DEBUG=false
DOCKER = docker run --rm -it -w /mnt -v ${PWD}:/mnt -e DEBUG=${DEBUG} rucc

check:
	cargo fmt
	$(DOCKER) cargo $@

.PHONY: build
build:
	cargo fmt
	$(DOCKER) cargo $@

debug: build
	cargo fmt
	$(DOCKER) rust-lldb target/debug/rucc DEBUG=true

run:
	cargo fmt
	$(DOCKER) cargo $@

tmp.s:
	$(DOCKER) cargo run>$@

tmp: tmp.s
	$(DOCKER) gcc -o tmp tmp.s

exec: tmp
	$(DOCKER) ./tmp
	echo $?

bash:
	$(DOCKER) $@

test:
	cargo fmt
	$(DOCKER) cargo $@
	$(DOCKER) sh test.sh
	make clean

coverage:
	$(DOCKER) sh coverage.sh

clean:
	rm -fv tmp*
