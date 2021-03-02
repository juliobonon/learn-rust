# About

A [Dockerfile](Dockerfile) to cross-compille gkt-rs apps for arm devices. Especially to run on Torizon.

If you want to build for armv7:

```
# docker build .
```

For armv8:

```
# docker build --build-arg PKG_ARCH=arm64 --build-arg TARGET=aarch64-unknown-linux-gnu --build-arg TARGET_IMAGE=arm64v8-debian-weston-vivante -t reininy/gtk-torizon-armv8
```

# Docker Images

[rustgtk](https://hub.docker.com/repository/docker/reininy/gtkrust) image on Dockerhub. 🐋
[gtk-torizon-armv8](https://hub.docker.com/repository/docker/reininy/gtk-torizon-armv8) image on Dockerhub. 🐋

