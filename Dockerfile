### ########### ###
## test
### ############
# TODO: #RUN cargo test --offline
# TODO: #RUN cargo bench --offline

### ########### ###
## build web
### ############
FROM node:latest

WORKDIR /code
COPY . .

RUN npm ci --silent
RUN npm run build:prod

# COPY openapi.yaml ./web/dist/openapi.yaml

### ########### ###
## build rust
### ############
FROM ekidd/rust-musl-builder:latest

# https://github.com/moby/moby/issues/4032#issuecomment-192327844
ARG DEBIAN_FRONTEND=noninteractive

# RUN cargo --version

COPY . .
COPY --from=0 /code/web/dist/ ./web/dist

RUN cargo build --release --all-features --quiet

### ########### ###
## create image
### ############
FROM alpine:latest

LABEL org.opencontainers.image.authors="Fabio Tea <iam@f4b.io>"
LABEL org.opencontainers.image.source="https://github.com/f4bio/nipplebot.rs"

ENV TZ=Etc/UTC
ENV APP_USER=appuser
ENV APP_LOG_LEVEL=warn

RUN apk update
RUN apk --no-cache add ca-certificates tzdata
RUN rm -rf /var/cache/apk/*

# useless?
RUN addgroup --system appuser
RUN adduser --system --home /app --ingroup appuser appuser

WORKDIR /app
COPY --from=1 /home/rust/src/target/x86_64-unknown-linux-musl/release/nipplebot /app/nipplebot
RUN chown -R appuser:appuser /app

USER appuser
EXPOSE 1337

ENTRYPOINT [ "/app/nipplebot" ]
