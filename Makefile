check:
	docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v ${PWD}:/mnt rucc cargo check

build:
	docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v ${PWD}:/mnt rucc cargo build

run:
	docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v ${PWD}:/mnt rucc cargo run

test:
	docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v ${PWD}:/mnt rucc sh test.sh
	make clean

clean:
	rm -fv tmp*
