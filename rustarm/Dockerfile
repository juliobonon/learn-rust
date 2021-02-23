FROM rustembedded/cross:armv7-unknown-linux-gnueabihf as build

ENV CARGO_HOME=/cargo
ENV PATH=/cargo/bin:$PATH
ENV PKG_CONFIG_PATH="/usr/lib/arm-linux-gnueabihf/pkgconfig"
ENV PKG_CONFIG_ALLOW_CROSS="true"
ENV USER=root

RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install 

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /usr/local/bin/rustup.sh
RUN bash /usr/local/bin/rustup.sh -y
RUN rustup default stable
RUN rustup target add x86_64-unknown-linux-gnu
RUN rustup target add armv7-unknown-linux-gnueabihf

COPY hello/ .
RUN cargo build --target=armv7-unknown-linux-gnueabihf --release
RUN ls -la

FROM torizon/arm32v7-debian-base
COPY --from=build /target/armv7-unknown-linux-gnueabihf/release/hello /usr/bin/

CMD /usr/bin/hello