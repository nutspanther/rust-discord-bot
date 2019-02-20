FROM rustlang/rust:nightly as build

ADD . /app

WORKDIR /app

RUN apk update \
&& apk add --no-cache \
  ca-certificates \
  ffmpeg \
  opus \
  python3 \
  libsodium-dev

RUN cargo update

RUN cargo build

CMD ["cargo","run"]