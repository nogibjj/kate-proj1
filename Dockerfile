FROM rust:latest as builder
ENV APP datavisualizer
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .
 
FROM debian:buster-slim

RUN apt-get update && \
    apt-get install -y libfontconfig1 && \
    rm -rf /var/lib/apt/lists/*
    
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP
# COPY ./data.txt /usr/local/bin/data.txt
ENTRYPOINT [ "/usr/local/bin/datavisualizer" ]