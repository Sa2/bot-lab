FROM rust:1.53-buster

RUN apt-get update -y && apt-get install -y gcc-aarch64-linux-gnu