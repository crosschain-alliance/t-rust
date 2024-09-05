FROM rust:slim-bookworm
RUN apt update -y && \
  apt install rsync -y && \
  mkdir /local_target
COPY run.sh /local_target
WORKDIR /local_target

CMD [ "debug" ]
