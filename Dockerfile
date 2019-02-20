FROM ubuntu:14.04
RUN apt-get update
RUN apt-get install -y python python-setuptools


 ADD . /shadowsocks	RUN apt-get update && apt-get install -y \
    python-software-properties \
    software-properties-common \
 && add-apt-repository ppa:chris-lea/libsodium \
 && echo "deb http://ppa.launchpad.net/chris-lea/libsodium/ubuntu trusty main" >> /etc/apt/sources.list \
 && echo "deb-src http://ppa.launchpad.net/chris-lea/libsodium/ubuntu trusty main" >> /etc/apt/sources.list \
 && apt-get update \
 && apt-get install -y libsodium-dev python-pip

FROM rust

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["cargo","run"]