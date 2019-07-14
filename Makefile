ARG=""

exec:
	docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v ${PWD}:/mnt rucc cargo run ${ARG}

check:
	docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v ${PWD}:/mnt rucc cargo check
