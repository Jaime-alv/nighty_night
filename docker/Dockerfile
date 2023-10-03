FROM lukemathwalker/cargo-chef:latest-rust-1.70 AS chef
WORKDIR /nighty_night

# STAGE 1 - generate a recipe file for dependencies
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# STAGE 2 - build source with cached dependencies
FROM chef AS builder
COPY --from=planner /nighty_night/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

# STAGE 3 - build docker image
FROM debian:bullseye-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates libpq5 \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 3000

ENV APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /nighty_night/target/release/nighty_night ${APP}/nighty_night

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./nighty_night"]