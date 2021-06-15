FROM gitpod/workspace-full

USER gitpod

RUN sudo apt-get update && \
    sudo apt-get install -y \
        rust-lldb \
    && sudo rm -rf /var/lib/apt/lists/*

ENV RUST_LLDB=/usr/bin/lldb-11