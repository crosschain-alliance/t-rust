FROM ubuntu:22.04
ENV TZ=Europe/London
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone && \
  apt update -y && \
  apt install rsync pkg-config libssl-dev curl build-essential gcc git wget clang -y && \
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup install nightly-2024-09-30-x86_64-unknown-linux-gnu
RUN cargo +nightly-2024-09-30 install --git https://github.com/a16z/jolt --force --bins jolt
RUN jolt install-toolchain
RUN mkdir /jolt_target
COPY run.sh /jolt_target
WORKDIR /jolt_target

CMD [ "release" ]
