FROM rustlang/rust:nightly as build

ADD src /app

WORKDIR /app

RUN cargo build

CMD ["cargo","run"]