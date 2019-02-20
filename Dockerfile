FROM rustlang/rust:nightly as build

ADD src /app

WORKDIR /app

RUN sudo apt-get install libsodium-dev

RUN cargo update

RUN cargo build

CMD ["cargo","run"]