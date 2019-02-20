FROM liuchong/rustup

RUN rustup default nightly

RUN cargo build

CMD ["cargo","run"]