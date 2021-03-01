FROM debian:bullseye as build

ENV CARGO_HOME=/cargo
ENV PATH=/cargo/bin:$PATH
ENV PKG_CONFIG_PATH="/usr/lib/arm-linux-gnueabihf/pkgconfig"
ENV PKG_CONFIG_ALLOW_CROSS="true"
ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER="/usr/bin/arm-linux-gnueabihf-gcc"
ENV USER=root

RUN dpkg --add-architecture armhf && \
    apt-get update && apt-get install -y build-essential cmake libc6-armhf-cross libc6-dev-armhf-cross \
    gcc-arm-linux-gnueabihf librust-gtk-dev libgtk-3-dev:armhf pkg-config \
    libdbus-1-dev libdbus-1-dev:armhf curl

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /usr/local/bin/rustup.sh
RUN bash /usr/local/bin/rustup.sh -y
RUN rustup default stable
RUN rustup target add x86_64-unknown-linux-gnu
RUN rustup target add armv7-unknown-linux-gnueabihf

COPY gtk-rs-app/ .
RUN cargo build --target=armv7-unknown-linux-gnueabihf --release
RUN ls /target/armv7-unknown-linux-gnueabihf/release/

FROM torizon/arm32v7-debian-base

RUN apt-get update && apt-get install && \
    apt-get install libgtk-3-dev

COPY --from=build /target/armv7-unknown-linux-gnueabihf/release/gtk-rs /usr/bin/

CMD /usr/bin/gtk-rs

