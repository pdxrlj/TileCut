FROM osgeo/gdal:ubuntu-small-3.3.1

COPY crate_config /root/.cargo/config
COPY crate_config /root/config

RUN sed -i s@/archive.ubuntu.com/@/mirrors.aliyun.com/@g /etc/apt/sources.list && \
    apt update && apt install pkg-config -y && \
    apt install cargo -y

