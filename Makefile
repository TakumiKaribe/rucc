DOCKER = docker run --rm -it -w /mnt -v ${PWD}:/mnt rucc

fmt:
	$(DOCKER) cargo $@

check:
	$(DOCKER) cargo $@

build:
	$(DOCKER) cargo $@

run:
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
	$(DOCKER) cargo $@
	$(DOCKER) sh test.sh
	make clean

coverage:
	$(DOCKER) sh coverage.sh

clean:
	rm -fv tmp*
