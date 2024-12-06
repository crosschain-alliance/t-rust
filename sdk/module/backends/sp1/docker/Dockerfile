FROM ubuntu:20.04
ENV TZ=Europe/London
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone && \
  apt update -y && \
  apt install rsync pkg-config libssl-dev curl build-essential gcc git wget clang -y && \
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl -L https://sp1.succinct.xyz | bash
ENV PATH="/root/.sp1/bin:${PATH}"
RUN wget -c https://go.dev/dl/go1.22.0.linux-amd64.tar.gz && \
    tar -C /usr/local/ -xzf go1.22.0.linux-amd64.tar.gz
ENV PATH="$PATH:/usr/local/go/bin"
RUN sp1up && \
  mkdir /sp1_target
COPY run.sh /sp1_target
WORKDIR /sp1_target

CMD [ "release" ]
