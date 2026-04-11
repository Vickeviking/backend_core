# Latest stable rust 
FROM lukemathwalker/cargo-chef:latest-rust-1.93.1-slim AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .

#compute a lock like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json

# Build project dependencies and not application

RUN cargo chef cook --release --recipe-path recipe.json

# Layers up to here stays cached if dependencie tree isnt touched

COPY . .
ENV SQLX_OFFLINE true

#Build project
RUN cargo build --release --bin api --bin worker

# Runtime stage
FROM debian:sid-slim AS runtime
WORKDIR /app
# install OpenSSL and ca-certificates to verify TLS certificates
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    #clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# copy over compiled binaries from builder phase
COPY --from=builder /app/target/release/api /app/api
COPY --from=builder /app/target/release/worker /app/worker

# copy over sourcefile dependencies, such as configuration file
COPY configuration configuration
COPY scripts/docker/entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

ENV APP_ENVIRONMENT production
ENV APP_RUNTIME api
ENTRYPOINT ["./entrypoint.sh"]
