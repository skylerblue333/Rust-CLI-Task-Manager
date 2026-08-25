FROM rust:1.98-bookworm AS builder
WORKDIR /app
COPY Cargo.toml ./
COPY src ./src
RUN cargo generate-lockfile && cargo build --release --locked

FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /data
COPY --from=builder /app/target/release/sky-tasks /usr/local/bin/sky-tasks
ENV SKY_TASKS_FILE=/data/tasks.json
USER nonroot:nonroot
ENTRYPOINT ["/usr/local/bin/sky-tasks"]
CMD ["help"]
