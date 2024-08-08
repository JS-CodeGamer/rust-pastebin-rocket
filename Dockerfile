FROM rust:alpine AS build

RUN apk add --no-cache musl-dev
WORKDIR /app
COPY ./Cargo.* /app/
COPY ./src /app/src
RUN cargo build --release


FROM alpine:latest

COPY ./Rocket.toml .
COPY --from=build /app/target/release/pastebin /bin/pastebin
EXPOSE 80
ENTRYPOINT ["/bin/pastebin"]
CMD ["--upload", "/pastes"]
