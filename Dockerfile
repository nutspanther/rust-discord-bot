FROM rust:1.32

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["cargo","run"]