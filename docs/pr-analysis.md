# Tentacle Open PR Analysis

Collected on 2026-07-06 from `nervosnetwork/tentacle` open PRs.

This document is the working analysis for the external integration-test
repository. The issue-sync version is kept in
`docs/acceptance-internal-1517-comment.md`.

## Open PR Groups

### QUIC integration

- #435 `quic: ServiceBuilder integration`
  - Adds public QUIC ServiceBuilder integration, docs, example, identity API cleanup, precise QUIC config errors, QUIC accept-loop resilience, and duplicate-dial parity.
  - Test focus: TCP and QUIC coexistence, `/udp/.../quic-v1` listen/dial, PeerId pin success/failure, `NotConfigured`/`Misconfigured` errors, bad-client does not kill listener.

### Connection lifecycle and shutdown

- #464 `Keep raw inbound handshakes alive during idle shutdown`
  - Raw inbound handshakes now track pending service state so `forever(false)` does not shut down before handshake completion.
  - Test focus: raw inbound session with no listeners/no active sessions remains alive until handshake resolves.
- #463 `Fix max connection limit off-by-one check`
  - Treat `active + pending == max_connection_number` as full; fail closed on overflow.
  - Test focus: exactly-at-capacity inbound/outbound attempts are rejected.
- #461 `Ignore late resource-creating events during shutdown`
  - Ignores late handshake/listen completions after `PreShutdown`.
  - Test focus: shutdown race with delayed handshake/listen completion creates no new sessions/listeners.
- #457 `Deduplicate dials only by authenticated peer identity`
  - Stops trusting pending `/p2p/<peer>` values for dial dedup; dedups only exact pending address or already-authenticated peer.
  - Test focus: spoofed pending `/p2p` dial cannot suppress legitimate dial.
- #436 `Enforce connection limits before inbound handshakes`
  - Counts inbound half-open handshakes against connection cap and releases permit on failure/close.
  - Implemented case: `stalled_inbound_connections_count_toward_limit_and_release_capacity`.

### Backpressure, cleanup, and resource accounting

- #449 `Enforce global budget for service task backpressure`
  - Adds shared budget across `ServiceControl` clones for counted service tasks while keeping control-plane tasks flowing.
  - Test focus: many control clones cannot exceed aggregate queued message budget; shutdown/close still works when budget full.
- #448 `Ensure substream errors always clean up session state`
  - Expected connection-close errors suppress user error events but still emit close cleanup.
  - Test focus: `BrokenPipe`/`UnexpectedEof` still removes substream/protocol state.
- #446 `Fix stale pending-byte accounting on dropped outbound messages`
  - Rolls back `pending_data_size` for messages dropped before substream acceptance or during cleanup.
  - Test focus: closed/missing protocol stream and abnormal-session cleanup leave pending bytes at zero.
- #443 `Backpressure small outbound messages by event count`
  - Adds event-count backpressure for tiny messages while allowing high-priority control events.
  - Test focus: full data queue defers `ProtocolMessage` but still processes `SessionClose`/`ProtocolClose`.
- #440 `Propagate substream write backpressure`
  - Stops draining data events into substream buffers while lower sink is pending, while preserving high-priority close.
  - Test focus: stalled write side still handles close promptly.

### Yamux and secio internals

- #458 `Ensure yamux streams notify session after half-close FIN`
  - `LocalClosing + FIN` now emits `StreamEvent::Closed` so session stream counts are cleaned.
  - Test focus: half-close then remote FIN frees stream slot and allows new stream.
- #455 `Wake write-only yamux streams on WindowUpdate`
  - Write-only stream registers writer waker so remote `WindowUpdate` wakes pending writes.
  - Test focus: split/write-only stream blocked on zero window resumes after `WindowUpdate`.
- #445 `Restore yamux write-stall timeout`
  - Tracks current write-stall start rather than previous successful send.
  - Test focus: idle connection does not false-timeout; actual persistent sink stall times out.
- #454 `refactor(secio): replace RecvBuf enum with Bytes`
  - Simplifies secio receive buffer to `Bytes`; review noted current unit tests only test upstream `Bytes::advance`.
  - Risk: add test that exercises `SecureStream::drain`/partial reads, not just `Bytes`.

### Transport and address handling

- #460 `Validate P2P bytes before serializing multiaddrs`
  - Validates `Protocol::P2P` bytes in constructors.
  - Implemented case: `invalid_p2p_is_rejected_by_safe_constructors`.
- #459 `Avoid local DNS resolution for SOCKS proxy dials`
  - Sends `/dns4`/`/dns6` hostnames to SOCKS proxy as ATYP domain.
  - Test focus: fake SOCKS5 server observes domain target, no local DNS lookup.
- #456 `Disable default SO_REUSEADDR on Tokio listeners`
  - Makes reuse-address opt-in via socket transformer.
  - Test focus: default listener socket reports `reuse_address == false` on Unix.
- #453 `Apply service frame limit to WebSocket handshakes`
  - Applies `max_frame_length` to tungstenite limits, accounting for default length-prefix overhead.
  - Test focus: WS accepts payload at configured limit + prefix and rejects +1.
- #451 `avoid CPU spin on short TCP peeks`
  - Draft. Author says current fix is not ideal and improving.
  - Test focus: short initial TCP input does not starve runtime and close/EOF logs are accurate.

## Immediate Risks

- #451 is draft and explicitly not final.
- #454 still needs a stronger partial-read regression.
- #453 has several comments around default framing overhead and test constant drift.
- Many PRs overlap on backpressure/control-event behavior; integration testing should run related PRs together, not only one-by-one.

## Local PR Refs

The local `nervosnetwork/tentacle` checkout has PR heads fetched as
`origin/pr/<number>`.

```bash
git fetch origin '+refs/pull/*/head:refs/remotes/origin/pr/*'
git diff --stat origin/master...origin/pr/464
git checkout -B pr-464 origin/pr/464
```
