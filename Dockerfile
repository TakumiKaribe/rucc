FROM ubuntu
FROM rust

LABEL maintainer "Takumi Karibe <takumi.k.5610@gmail.com>"

RUN apt-get -y update
RUN \
    apt-get -y install \
    build-essential less gdb \
    && apt-get clean
