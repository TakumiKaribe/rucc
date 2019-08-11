check:
	docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v ${PWD}:/mnt rucc cargo check

build:
	docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v ${PWD}:/mnt rucc cargo build

run:
	docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v ${PWD}:/mnt rucc cargo run

tmp.s:
	docker run --rm --privileged --security-opt="seccomp=unconfined" -i -w /mnt -v ${PWD}:/mnt rucc cargo run>tmp.s

tmp: tmp.s
	docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v ${PWD}:/mnt rucc gcc -o tmp tmp.s

exec: tmp
	docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v ${PWD}:/mnt rucc ./tmp
	echo $?

shell:
	docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v ${PWD}:/mnt rucc bash

test:
	docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v ${PWD}:/mnt rucc sh test.sh
	make clean

clean:
	rm -fv tmp*
