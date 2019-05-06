if [ -z "$(docker image ls -q rucc)" ]; then
    docker build . -t rucc
fi

docker run --rm --privileged --security-opt="seccomp=unconfined" -it -w /mnt -v $PWD:/mnt rucc bash
