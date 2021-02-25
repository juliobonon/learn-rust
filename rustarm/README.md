# About

An armhf binary to get system information written in rust..

You can run it directly from Docker Hub: 🐋

```
docker run reininy/rustarm
```

You can build it for armv7 with the following command:

```
docker build .
```

For armv8:

```
docker build --build-arg IMAGE_ARCH=aarch64-unknown-linux-gnu --build-arg PKG_ARCH=aarch64 --build-arg TARGET_IMAGE=arm64v8-debian-base .
```

# Output

```
Hello, world!
os: Linux 5.4.77-5.2.0-devel+git.1c6041797725
cpu: 2 cores, 7 MHz
proc total: 152
hostname: 5286f417d054
mem: total 503956 KB, free 132144 KB, avail 328684 KB, buffers 11980 KB, cached 203128 KB
```
