FROM ubuntu:14.04
RUN apt-get update

RUN apt-get install -y software-properties-common \
    && bash -c "LC_ALL=C.UTF-8 add-apt-repository -y ppa:ondrej/php" \
    && apt-get update \
    && apt-get install -y libsodium-dev

FROM rust

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["cargo","run"]