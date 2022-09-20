# Build Stage
FROM debian as builder

WORKDIR /
RUN apt update -y && apt install -y g++ gcc git curl wget nasm yasm libgtk-3-dev clang libxcb-randr0-dev libxdo-dev libxfixes-dev libxcb-shape0-dev libxcb-xfixes0-dev libasound2-dev libpulse-dev cmake unzip zip sudo

RUN git clone https://github.com/microsoft/vcpkg && cd vcpkg && git checkout 134505003bb46e20fbace51ccfb69243fbbc5f82
RUN /vcpkg/bootstrap-vcpkg.sh -disableMetrics
RUN /vcpkg/vcpkg --disable-metrics install libvpx libyuv opus
ENV VCPKG_ROOT=/vcpkg

#RUN groupadd -r user && useradd -r -g user user --home /home/user && mkdir -p /home/user && chown user /home/user && echo "user  ALL=(ALL) NOPASSWD:ALL" | sudo tee /etc/sudoers.d/user
#WORKDIR /home/user
#RUN wget https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.lnx/x64/libsciter-gtk.so
#USER user
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh
RUN chmod +x rustup.sh
RUN ./rustup.sh -y --default-toolchain nightly

#USER root

#FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd libs/scrap/fuzz && ${HOME}/.cargo/bin/cargo +nightly -Z sparse-registry build --release && popd" && \
    mv libs/scrap/fuzz/target/release/vpxdecode /vpxdecode

# Package Stage
FROM debian

COPY --from=builder /vpxdecode /
COPY --from=builder \
    /usr/lib/x86_64-linux-gnu/libxcb.so.1 \
    /usr/lib/x86_64-linux-gnu/libxcb-shm.so.0 \
    /usr/lib/x86_64-linux-gnu/libxcb-randr.so.0 \
    /usr/lib/x86_64-linux-gnu/libXau.so.6 \
    /usr/lib/x86_64-linux-gnu/libXdmcp.so.6 \
    /usr/lib/x86_64-linux-gnu/libbsd.so.0 \
    /usr/lib/x86_64-linux-gnu/libmd.so.0 \
    /usr/lib/x86_64-linux-gnu/
