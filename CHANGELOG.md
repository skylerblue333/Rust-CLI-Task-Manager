# Changelog

## 0.1.0 - 2026-08-24

- replaced the mismatched Actix HTTP service with a real local CLI task manager
- added bounded task/title validation and deterministic numeric IDs
- added add/list/complete/remove commands
- added atomic JSON persistence with explicit storage-path configuration
- added unit coverage, strict rustfmt/clippy/test/build gates, dependency audit, and CLI smoke testing
- added non-root distroless container packaging
- documented engineering-beta status and security boundaries
