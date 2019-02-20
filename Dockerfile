FROM liuchong/rustup

RUN rustup default nightly

ADD src /app

WORKDIR /app

RUN cargo build

CMD ["cargo","run"]