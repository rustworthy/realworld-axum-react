FROM docker.io/rust:1-slim-bookworm AS build

WORKDIR /build

COPY Cargo.* ./
COPY src ./src
COPY static ./static
COPY templates ./templates
COPY migrations ./migrations
COPY .sqlx ./.sqlx

ENV SQLX_OFFLINE=1

RUN apt update && apt install -y protobuf-compiler

RUN --mount=type=cache,target=/build/target \
  --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/usr/local/cargo/git \
  set -eux; \
  cargo build --release; \
  objcopy --compress-debug-sections target/release/realworld-axum-react ./main

################################################################################

FROM docker.io/debian:bookworm-slim

# https://www.docker.com/blog/understanding-the-docker-user-instruction/
RUN groupadd --gid 1234 app
RUN useradd --uid 1234 --create-home --gid app app

WORKDIR /home/app

COPY --from=build /build/main ./
COPY --from=build /build/stati[c] ./static
COPY --from=build /build/template[s] ./templates
COPY --from=build /build/migration[s] ./migrations

RUN chown -R app:app .

USER app

CMD ["./main"]
