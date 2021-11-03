FROM rust:1.56

WORKDIR /FIRST-TOKIO

COPY . /app
RUN cargo install --path /app

CMD ["vdbmgr"]