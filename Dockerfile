FROM rust:1.88-alpine as builder
WORKDIR /app

RUN apk add musl-dev

COPY . .
RUN cargo build --release

RUN mkdir -p /opt/resource && \
    cp /app/target/release/concourse-ntfy-resource /opt/resource/concourse-ntfy-resource && \
    ln -s /opt/resource/concourse-ntfy-resource /opt/resource/check && \
    ln -s /opt/resource/concourse-ntfy-resource /opt/resource/in && \
    ln -s /opt/resource/concourse-ntfy-resource /opt/resource/out

FROM scratch AS runtime
WORKDIR /app

COPY --from=builder /opt/resource /opt/resource
ENTRYPOINT ["/opt/resource/out"]