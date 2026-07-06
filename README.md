# Tentacle Integration Tests

External integration-test harness for `nervosnetwork/tentacle` open PR validation.

This repository is intentionally outside the main `tentacle` workspace. It verifies behavior through the public API where possible, and keeps PR-specific analysis in `docs/` so failures can be referenced from GitHub issues.

## Local Usage

By default `Cargo.toml` points at the local checkout:

```text
../../nervosnetwork/tentacle/tentacle
```

To validate a PR:

```bash
cd /Users/xue/nervosnetwork/tentacle
git fetch origin pull/<PR_NUMBER>/head:pr-<PR_NUMBER>
git checkout pr-<PR_NUMBER>

cd /Users/xue/Xcodes-chain/tentacle-integration-tests
cargo test --features ws,quic -- --nocapture
```

Run one case:

```bash
./scripts/run-pr-case.sh stalled_inbound_connections_count_toward_limit_and_release_capacity
```

## Case Map

- `tcp_service_can_exchange_burst_messages`: baseline TCP + secio + yamux message delivery.
- `stalled_inbound_connections_count_toward_limit_and_release_capacity`: PR nervosnetwork/tentacle#436 inbound half-open connection-limit behavior, ignored by default.
- `invalid_p2p_is_rejected_by_safe_constructors`: PR nervosnetwork/tentacle#460 multiaddr P2P invariant, ignored by default.
- `quic_service_can_exchange_burst_messages`: PR nervosnetwork/tentacle#435 QUIC ServiceBuilder smoke coverage.
- `websocket_rejects_message_over_service_frame_limit`: PR nervosnetwork/tentacle#453 WebSocket frame-limit behavior, ignored by default.
- `websocket_accepts_message_at_service_frame_limit_plus_prefix`: WebSocket at-limit control case.
- `tests/planned_pr_cases.rs`: ignored skeletons for the remaining open PRs, used as stable references when filing upstream issues.

Planned next cases are tracked in `docs/test-plan.md`.

## Sync Docs

- `docs/pr-analysis.md`: working PR analysis.
- `docs/test-plan.md`: integration matrix and issue template.
- `docs/acceptance-internal-1517-comment.md`: paste-ready sync comment for `nervosnetwork/acceptance-internal#1517`.
- `docs/validation-results-2026-07-06.md`: local validation results for open, non-draft, CI-success tentacle PRs.
- `docs/external-case-results-2026-07-06.md`: external integration case results against master and selected PR refs.
