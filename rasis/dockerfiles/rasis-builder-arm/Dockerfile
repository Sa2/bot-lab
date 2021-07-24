FROM ubuntu:20.04


RUN apt-get update -y && \
    apt-get install -y curl gcc-aarch64-linux-gnu && \
    apt-get clean && \
    apt-get autoremove && \
    apt-get autoclean && \
    # curl https://sh.rustup.rs -sSf | sh -s -- -y --default-host aarch64-unknown-linux-gnu --default-toolchain stable
    curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
    # rustup install stable-aarch64-unknown-linux-gnu && \
    # cargo install cross

ENV PATH $PATH:$HOME/.cargo/bin

# RUN cargo install cross