# Security Policy

## Supported status

Sky CLI Tasks is an engineering-beta local CLI. It has no network listener, authentication layer, tenant isolation, or remote execution feature.

## Security boundaries

- Task titles are treated strictly as data and are never executed.
- The configured JSON task file may contain user-sensitive text; protect it with host filesystem permissions and encryption as appropriate.
- Persistence uses a temporary file plus rename, but this is not a substitute for backups or transactional storage.
- The CLI rejects malformed task files and bounded task/title sizes to reduce accidental resource exhaustion.
- Container execution uses a non-root runtime identity.

Report suspected vulnerabilities privately through GitHub's security reporting channel when available. Do not include real secrets or private task data in public issues.
