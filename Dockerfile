FROM rust:alpine3.10
ENV RUSTFLAGS="-C target-feature=-crt-static"
WORKDIR /usr/src/rebuilderd
RUN apk add --no-cache musl-dev openssl-dev
COPY . .
RUN cd daemon; cargo build --release
RUN cd tools; cargo build --release

FROM alpine:3.10
ENV HTTP_ADDR=0.0.0.0:8080
RUN apk add --no-cache libgcc openssl dpkg
COPY --from=0 \
    /usr/src/rebuilderd/target/release/rebuilderd \
    /usr/src/rebuilderd/target/release/rebuildctl \
    /usr/local/bin/
CMD ["rebuilderd"]
