FROM rust:latest AS backend-builder

WORKDIR /usr/src/server

COPY ../share /usr/src/share
COPY ../server /usr/src/server

RUN cargo b

ENV ROCKET_ADDRESS=0.0.0.0

EXPOSE 8000

COPY --from=backend-builder /usr/src/server/target/release/server /usr/bin/server

CMD ["/usr/src/app/target/debug/server"]