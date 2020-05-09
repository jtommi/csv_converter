FROM rust:slim-buster

WORKDIR /usr/src/myapp

RUN apt-get -y update && \
    apt-get -y upgrade && \
    apt install -y git && \
    apt-get autoclean

CMD ["/bin/bash"]
