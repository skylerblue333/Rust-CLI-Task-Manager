# Sky CLI Tasks

A focused local task-management CLI in Rust for the SKYCOIN4444 engineering lab.

## Status

**Engineering beta.** The CLI supports bounded local task creation, deterministic numeric IDs, completion/removal, atomic JSON persistence, strict CI gates, dependency audit, and non-root container packaging. It does **not** claim multi-user collaboration, cloud sync, encryption at rest, distributed coordination, or production deployment.

## Commands

```bash
sky-tasks add "ship release notes"
sky-tasks list
sky-tasks complete 1
sky-tasks remove 1
```

The storage path defaults to `./tasks.json`. Override it with:

```bash
export SKY_TASKS_FILE=/safe/path/tasks.json
```

## Data and safety boundaries

- task titles must contain 1–200 characters
- at most 10,000 tasks are loaded or created
- IDs are monotonically allocated from the highest existing ID
- writes go to a temporary file, are synced, then renamed over the target file
- malformed JSON fails closed instead of silently discarding data
- the application never executes task titles as shell commands

Atomic rename semantics are filesystem-dependent. Keep the task file on a local filesystem and maintain backups if the data matters.

## Verification

Requires Rust 1.98 for the declared CI baseline.

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo build --release
```

GitHub Actions additionally runs `cargo audit`, a CLI smoke test, a Docker build, and verifies that the runtime image is non-root.

## Container

```bash
docker build -t sky-cli-tasks .
docker run --rm -v "$PWD/data:/data" sky-cli-tasks add "container task"
docker run --rm -v "$PWD/data:/data" sky-cli-tasks list
```

The runtime uses the distroless `nonroot` identity. Mount `/data` writable if persistence is required.

## SKYCOIN4444 integration

Keep this repository independently reusable. If the broader ecosystem needs task data, integrate through an adapter that reads/writes an explicit task contract rather than copying the CLI implementation into a flagship application.

## Security

Task titles are data only and are never executed. Treat the JSON file as user data; filesystem permissions, backup policy, and host encryption remain deployment responsibilities. See `SECURITY.md`.

## License

See `LICENSE`.
