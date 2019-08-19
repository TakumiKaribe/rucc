DOCKER = docker run --rm -it -w /mnt -v ${PWD}:/mnt rucc
DEBUG=false

check:
	cargo fmt
	$(DOCKER) cargo $@

build:
	cargo fmt
	$(DOCKER) cargo $@

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
	$(DOCKER) sh test.sh ${DEBUG}
	make clean

coverage:
	$(DOCKER) sh coverage.sh

clean:
	rm -fv tmp*
