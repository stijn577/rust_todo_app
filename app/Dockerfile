# Stage 1: Build the Rust application
FROM rust:latest as frontend-builder

WORKDIR /usr/src/app

COPY ../app /usr/src/app
COPY ../share /usr/src/share

RUN cargo install trunk --locked
RUN rustup target add wasm32-unknown-unknown

RUN trunk build

# Stage 2: Setup Nginx
FROM nginx:alpine
COPY --from=frontend-builder /usr/src/dist /usr/share/nginx/html
COPY ./app/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]