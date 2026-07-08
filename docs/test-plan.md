# Tentacle Integration Test Plan

## Goal

Validate the open PR set against realistic service-level behavior:

- connection admission and shutdown races
- control-plane liveness under data backpressure
- pending-byte/resource cleanup
- transport-specific limits and identity checks
- QUIC parity with TCP service APIs

## Strategy

1. Keep reusable harness code in `src/lib.rs`.
2. Prefer public `ServiceBuilder`, `ServiceControl`, `ServiceHandle`, and protocol callbacks.
3. Use local loopback sockets and deterministic timeouts.
4. Run each PR branch through the same case set.
5. When a problem is found, open an issue in `nervosnetwork/tentacle` that references:
   - the PR number
   - this repository
   - the failing test name
   - observed logs / assertion

## Test Matrix

| Area | Cases | Related PRs | Status |
|---|---|---:|---|
| Baseline service | TCP secio/yamux burst delivery | all | implemented |
| Connection cap | stalled inbound half-open sockets count and release | nervosnetwork/tentacle#436 | implemented |
| Connection cap | equality boundary and overflow fail-closed | nervosnetwork/tentacle#463 | planned |
| Multiaddr | invalid raw P2P constructors panic; valid P2P round-trip | nervosnetwork/tentacle#460 | implemented |
| Dial identity | spoofed pending `/p2p` does not block legitimate dial | nervosnetwork/tentacle#457 | planned |
| Service budget | many `ServiceControl` clones share budget; close still flows | nervosnetwork/tentacle#449 | planned |
| Substream write pressure | lower sink pending does not drain data but handles close | nervosnetwork/tentacle#440, nervosnetwork/tentacle#443, nervosnetwork/tentacle#448 | planned |
| Pending bytes | dropped outbound messages return pending byte counter to zero | nervosnetwork/tentacle#446 | planned |
| Yamux | WindowUpdate wakes write-only stream; half-close FIN cleans slot; write-stall timeout | nervosnetwork/tentacle#455, nervosnetwork/tentacle#458, nervosnetwork/tentacle#445 | planned |
| Session idle timeout | timeout re-arms after the first check fires while protocol substreams are still active | nervosnetwork/tentacle#465 | implemented |
| WebSocket | configured max frame + prefix accepted; +1 rejected | nervosnetwork/tentacle#453 | implemented |
| SOCKS DNS | `/dns4` proxied dial sends ATYP domain | nervosnetwork/tentacle#459 | planned |
| TCP peek | short initial input does not spin runtime | nervosnetwork/tentacle#451 | planned after draft stabilizes |
| QUIC | listen/dial burst smoke | nervosnetwork/tentacle#435 | implemented |
| QUIC | peer-id pin success/failure, bad handshake does not kill listener | nervosnetwork/tentacle#435 | planned |

The remaining planned cases are represented by ignored test skeletons in
`tests/planned_pr_cases.rs` so upstream issues can reference stable case names
before the full reproduction logic is implemented.

## Commands

Validate all currently implemented cases:

```bash
cargo test --features ws,quic -- --nocapture
```

Validate one PR branch:

```bash
cd /Users/xue/nervosnetwork/tentacle
git fetch origin pull/436/head:pr-436
git checkout pr-436

cd /Users/xue/Xcodes-chain/tentacle-integration-tests
cargo test stalled_inbound_connections_count_toward_limit_and_release_capacity -- --ignored --nocapture
```

## Issue Template For Findings

```markdown
## Summary

<one sentence>

## Affected PR / module

- PR: #<number>
- Module: <service/yamux/secio/quic/ws/etc>
- Integration case: Xcodes-chain/tentacle-integration-tests::<test_name>

## Reproduction

```bash
cd /path/to/tentacle
git checkout pr-<number>
cd /path/to/tentacle-integration-tests
cargo test <test_name> -- --nocapture
```

## Expected

<expected behavior>

## Actual

<actual behavior/log/assertion>

## Notes

<suspected root cause, if known>
```
