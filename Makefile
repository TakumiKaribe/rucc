DEBUG=false
DOCKER = docker run --rm -it -w /mnt -v ${PWD}:/mnt -e DEBUG=${DEBUG} rucc

.PHONY: fmt
fmt:
	cargo fmt

check: fmt
	$(DOCKER) cargo $@

.PHONY: build
build: fmt
	$(DOCKER) cargo $@

debug: fmt build
	$(DOCKER) rust-lldb target/debug/rucc

run: fmt
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

test: fmt
	$(DOCKER) cargo $@
	$(DOCKER) sh test.sh
	make clean

coverage:
	$(DOCKER) sh coverage.sh

clean:
	rm -fv tmp*
