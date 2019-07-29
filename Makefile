# you can run run with arguments.
# `make run ARG=xxxx`
ARG=""

build:
	docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v ${PWD}:/mnt rucc cargo build

check:
	docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v ${PWD}:/mnt rucc cargo check

run:
	docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v ${PWD}:/mnt rucc cargo run "${ARG}"

test:
	docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v ${PWD}:/mnt rucc sh test.sh
	make clean

clean:
	rm -fv tmp*
