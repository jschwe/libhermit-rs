FROM mcr.microsoft.com/vscode/devcontainers/rust:1

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends  make cmake qemu-kvm qemu-system-x86 nasm gcc g++ build-essential libtool bsdmainutils pkg-config libssl-dev lld
