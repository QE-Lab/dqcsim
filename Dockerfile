FROM debian:buster

ENV TERM linux
ENV HOME /root
ENV PATH="${HOME}/.cargo/bin:${PATH}"
ENV DQCSIM_HOME="${HOME}/.dqcsim"

RUN apt-get update && \
    apt-get install -y \
      curl \
      cmake \
      g++ \
      git \
      python3 \
      python3-dev \
      python3-setuptools \
      swig \
    && rm -rf /var/lib/apt/lists/* && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    rustup component add clippy && \
    rustup component add rustfmt && \
    cargo install cargo-make

COPY . .

RUN make test
