DEBUG=false
TARGET=""
DOCKER = docker run --rm -it -w /mnt -v ${PWD}:/mnt -e DEBUG=${DEBUG} rucc

.PHONY: fmt
fmt:
	$(DOCKER) cargo $@

.PHONY: clippy
clippy:
	$(DOCKER) cargo $@

check: fmt clippy
	$(DOCKER) cargo $@

.PHONY: build
build: fmt clippy
	$(DOCKER) cargo $@

debug: fmt clippy build
	$(DOCKER) rust-lldb target/debug/rucc

run: fmt clippy
	$(DOCKER) cargo $@ $(TARGET)

tmp.s:
	$(DOCKER) cargo run>$@

tmp: tmp.s
	$(DOCKER) gcc -o tmp tmp.s

exec: tmp
	$(DOCKER) ./tmp
	echo $?

bash:
	$(DOCKER) $@

test: fmt clippy
	$(DOCKER) cargo $@
	$(DOCKER) sh test.sh
	make clean

coverage:
	$(DOCKER) sh coverage.sh

clean:
	rm -fv tmp*
