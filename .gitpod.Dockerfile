FROM gitpod/workspace-full

USER gitpod

# LLVM config
RUN sudo apt-get update && \
    sudo apt-get install -y \
        rust-lldb \
    && sudo rm -rf /var/lib/apt/lists/*

# Clippy install
RUN rustup component add clippy

ENV RUST_LLDB=/usr/bin/lldb-11