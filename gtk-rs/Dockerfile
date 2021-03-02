ARG PKG_ARCH=armhf
# For arm64 use:
# ARG PKG_ARCH=armv8
ARG TARGET=armv7-unknown-linux-gnueabihf
# For arm64 use:
# ARG TARGET=aarch64-unknown-linux-gnu
ARG TARGET_IMAGE=arm32v7-debian-wayland-base
# For arm64 use:
# ARG TARGET_IMAGE=arm64v8-debian-weston-vivante:bullseye
FROM debian:bullseye as build

ARG PKG_ARCH
ARG TARGET
ARG TARGET_IMAGE

ENV CARGO_HOME=/cargo
ENV PATH=/cargo/bin:$PATH
ENV PKG_CONFIG_PATH="/usr/lib/arm-linux-gnueabihf/pkgconfig"
ENV PKG_CONFIG_ALLOW_CROSS="true"
ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER="/usr/bin/arm-linux-gnueabihf-gcc"
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER="/usr/bin/aarch64-linux-gnu-gcc"
ENV USER=root

RUN dpkg --add-architecture ${PKG_ARCH} && \
    apt-get update && apt-get install -y build-essential cmake libc6-${PKG_ARCH}-cross libc6-dev-${PKG_ARCH}-cross \
    gcc-arm-linux-gnueabihf g++-aarch64-linux-gnu librust-gtk-dev libgtk-3-dev:${PKG_ARCH} pkg-config \
    libdbus-1-dev libdbus-1-dev:${PKG_ARCH} curl

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /usr/local/bin/rustup.sh
RUN bash /usr/local/bin/rustup.sh -y
RUN rustup default stable
RUN rustup target add x86_64-unknown-linux-gnu
RUN rustup target add ${TARGET}

COPY gtk-rs-app/ .
RUN cargo build --target=${TARGET} --release
RUN ls /target/

FROM torizon/${TARGET_IMAGE}

ENV DEBIAN_FRONTEND="noninteractive"
ENV ACCEPT_FSL_EULA=1

RUN apt-get update && apt-get install -y && \
    apt-get install libgtk-3-0

COPY --from=build /target/a*/release/gtk-rs /usr/bin/

CMD /usr/bin/gtk-rs

