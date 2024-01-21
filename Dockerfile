FROM rust:1.75-slim-buster

WORKDIR /usr/src/fife-bin-calendar
COPY . .

RUN apt-get update -y && \
      apt-get install -y pkg-config libssl-dev

RUN cargo install --path .

EXPOSE 8000

CMD ["fife-bin-calendar"]