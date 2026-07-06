# Tentacle Open PR Analysis

Collected on 2026-07-06 from `nervosnetwork/tentacle` open PRs.

This document follows the actual open PRs, review state, CI state, and changed
files from `nervosnetwork/tentacle`. The paste-ready sync version is
`docs/acceptance-internal-1517-comment.md`.

## State Summary

- Open PRs: 21.
- Draft: nervosnetwork/tentacle#451.
- Approved: nervosnetwork/tentacle#435, nervosnetwork/tentacle#454.
- Changes requested: nervosnetwork/tentacle#443, nervosnetwork/tentacle#445.
- CI test failures at collection time: nervosnetwork/tentacle#436, nervosnetwork/tentacle#449.
- The rest are review-required with CI passing.

## PR-by-PR Analysis

| PR | Review / CI | Changed area | External validation focus |
|---:|---|---|---|
| nervosnetwork/tentacle#435 | Approved, CI pass | QUIC docs/example public usage | QUIC listen/dial, PeerId pin success/failure, bad client does not kill listener, clear NotConfigured/Misconfigured errors. |
| nervosnetwork/tentacle#436 | Review required, CI Test fail | service connection cap before inbound handshake | Reproduce CI failure; stalled inbound sockets count toward max and release capacity after close/failure. |
| nervosnetwork/tentacle#440 | Review required, CI pass | substream write backpressure | Data writes stop draining while lower sink pending; close/control events remain live. |
| nervosnetwork/tentacle#443 | Changes requested, CI pass | session/quic event-count backpressure | Backpressure only data messages; `SessionClose`/`ProtocolClose` must not be blocked by full small-message queues. |
| nervosnetwork/tentacle#445 | Changes requested, CI pass | yamux write-stall timeout | Idle connection must not timeout on first transient pending; sustained write stall must timeout. |
| nervosnetwork/tentacle#446 | Review required, CI pass | pending byte accounting | Dropped outbound messages and cleanup paths decrement pending bytes. |
| nervosnetwork/tentacle#448 | Review required, CI pass | substream expected-close cleanup | Common close errors suppress user error but still clean session/protocol state. |
| nervosnetwork/tentacle#449 | Review required, CI Test fail | global service-task budget | Reproduce CI failure; cloned controls share one budget while control events still pass. |
| nervosnetwork/tentacle#450 | Review required, CI pass | protocol task cancellation | Pending service/session protocol callback future is dropped on cancel/shutdown. |
| nervosnetwork/tentacle#451 | Draft, CI pass | TCP short peek loop | Track only until ready; validate no CPU spin after draft stabilizes. |
| nervosnetwork/tentacle#453 | Review required, CI pass | WebSocket frame limit plumbing | WS parser rejects oversize according to service frame limit and length-prefix overhead. |
| nervosnetwork/tentacle#454 | Approved, CI pass | secio receive buffer | SecureStream partial reads/drain preserve exact bytes; avoid testing only `Bytes::advance`. |
| nervosnetwork/tentacle#455 | Review required, CI pass | yamux WindowUpdate wake | Write-only stream blocked by zero window wakes on WindowUpdate without read polling. |
| nervosnetwork/tentacle#456 | Review required, CI pass | Tokio listener socket option | Default listener does not enable SO_REUSEADDR; transformer opt-in still works. |
| nervosnetwork/tentacle#457 | Review required, CI pass | dial dedup peer identity | Unauthenticated pending `/p2p` cannot suppress legitimate dial; authenticated dedup remains. |
| nervosnetwork/tentacle#458 | Review required, CI pass | yamux half-close cleanup | `LocalClosing + FIN` emits close to parent session and releases stream slot. |
| nervosnetwork/tentacle#459 | Review required, CI pass | SOCKS DNS dial path | Proxied DNS dial sends domain target to SOCKS server, no local DNS resolution. |
| nervosnetwork/tentacle#460 | Review required, CI pass | multiaddr P2P construction | Invalid raw P2P bytes rejected by construction paths; valid PeerId round-trips. |
| nervosnetwork/tentacle#461 | Review required, CI pass | service shutdown late events | PreShutdown rejects/cleans late handshake and listen completions. |
| nervosnetwork/tentacle#463 | Review required, CI pass | connection limit boundary | Equality boundary is full; overflow fails closed. |
| nervosnetwork/tentacle#464 | Review required, CI pass | raw inbound idle shutdown | Raw inbound handshake registers pending work so `forever(false)` does not exit early. |

## Current Validation Scope

This round validates only open, non-draft PRs whose CI is currently successful.

Included:

- nervosnetwork/tentacle#435, nervosnetwork/tentacle#440, nervosnetwork/tentacle#443, nervosnetwork/tentacle#445, nervosnetwork/tentacle#446, nervosnetwork/tentacle#448, nervosnetwork/tentacle#450, nervosnetwork/tentacle#453, nervosnetwork/tentacle#454, nervosnetwork/tentacle#455, nervosnetwork/tentacle#456, nervosnetwork/tentacle#457, nervosnetwork/tentacle#458, nervosnetwork/tentacle#459, nervosnetwork/tentacle#460, nervosnetwork/tentacle#461, nervosnetwork/tentacle#463, nervosnetwork/tentacle#464

Excluded from this round:

- nervosnetwork/tentacle#436: open and non-draft, but CI Test is failing.
- nervosnetwork/tentacle#449: open and non-draft, but CI Test is failing.
- nervosnetwork/tentacle#451: draft.

Execution priority:

1. Run each PR's own targeted test command on its PR ref.
2. Add external integration coverage where the PR touches public behavior.
3. Treat nervosnetwork/tentacle#443 and nervosnetwork/tentacle#445 as CI-success but review-blocked; record the reviewer risk separately.
4. Revisit nervosnetwork/tentacle#436 / nervosnetwork/tentacle#449 as CI-failure reproduction tasks after this round.

Local validation results for this scope are recorded in
`docs/validation-results-2026-07-06.md`.

## Local PR Refs

The local `nervosnetwork/tentacle` checkout has PR heads fetched as
`origin/pr/<number>`.

```bash
git fetch origin '+refs/pull/*/head:refs/remotes/origin/pr/*'
git diff --stat origin/master...origin/pr/464
git checkout -B pr-464 origin/pr/464
```
