FROM rustembedded/cross:armv7-unknown-linux-gnueabihf as build

ENV CARGO_HOME=/cargo
ENV PATH=/cargo/bin:$PATH
ENV PKG_CONFIG_PATH="/usr/lib/arm-linux-gnueabihf/pkgconfig"
ENV PKG_CONFIG_ALLOW_CROSS="true"
ENV USER=root

RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install 

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /usr/local/bin/rustup.sh && \
bash /usr/local/bin/rustup.sh -y && \
rustup default stable && \
rustup target add x86_64-unknown-linux-gnu && \
rustup target add armv7-unknown-linux-gnueabihf

COPY hello/ .
RUN cargo build --target=armv7-unknown-linux-gnueabihf --release

FROM torizon/arm32v7-debian-base
COPY --from=build /target/armv7-unknown-linux-gnueabihf/release/hello /usr/bin/

CMD /usr/bin/hello