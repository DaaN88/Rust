FROM alpine:latest

ENV PYTHONUNBUFFERED=1

RUN apk update && apk add --no-cache \
    nano \
    vim \
    sudo \
    cmake \
    git \
    perl \
    unzip \
    tar \
    curl \
    build-base gcc g++ \
    libc-dev \
    boost-dev \
    boost-static \
    python3 \
    gcc \
    musl-dev \
    rust \
    cargo

COPY . /src

WORKDIR /src
