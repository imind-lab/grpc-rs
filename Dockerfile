####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN apt-get update && apt-get install -y protobuf-compiler pkg-config libssl-dev gcc make gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu
RUN update-ca-certificates

ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /app

COPY ./ .

RUN rm -rf target

RUN cargo build --release -p micro

####################################################################################################
## Final image
####################################################################################################

FROM debian:buster-slim
ARG APP=/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8088

ENV TZ=Etc/UTC \
    APP_USER=app

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /app/target/release/micro ${APP}/micro
COPY .env ${APP}/.env

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./micro"]
