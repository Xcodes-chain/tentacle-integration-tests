# Validation Results 2026-07-06

Scope: `nervosnetwork/tentacle` open PRs that are non-draft and whose GitHub CI was successful at collection time.

Excluded from this round:

- #436: open/non-draft, but GitHub CI Test was failing.
- #449: open/non-draft, but GitHub CI Test was failing.
- #451: draft.

Environment notes:

- Local checkout: `/Users/xue/nervosnetwork/tentacle`.
- PR refs: `origin/pr/<number>`.
- Commands that build `tentacle` or `tentacle-secio` used `openssl-vendored` to avoid local `pkg-config`/OpenSSL dependency issues on macOS.
- `tentacle` was returned to `master` after validation.

## Summary

All 18 PRs in scope passed their local targeted validation commands.

| PR | Local branch | Command | Result | Notes |
|---:|---|---|---|---|
| #435 | `verify-pr-435` | `cargo test --features quic,openssl-vendored -p tentacle --test test_quic -- --nocapture` | pass | 6 tests passed. |
| #435 | `verify-pr-435` | `cargo test --features quic,openssl-vendored -p tentacle --test test_quic_verifier -- --nocapture` | pass | 2 tests passed. |
| #435 | `verify-pr-435` | `cargo build --features quic,openssl-vendored -p tentacle --example quic_simple` | pass | Example builds. |
| #440 | `verify-pr-440` | `cargo test -p tentacle --features openssl-vendored -- --nocapture` | pass | Full package tests passed. Includes `write_pending_still_handles_high_priority_close`. |
| #443 | `verify-pr-443` | `cargo test -p tentacle --features openssl-vendored -- --nocapture` | pass | Full package tests passed. Review state remains changes requested. |
| #445 | `verify-pr-445` | `cargo test -p tokio-yamux -- --nocapture` | pass | 22 tests passed. Review state remains changes requested. |
| #446 | `verify-pr-446` | `cargo test -p tentacle --features openssl-vendored -- --nocapture` | pass | Full package tests passed. |
| #448 | `verify-pr-448` | `cargo test -p tentacle --features openssl-vendored expected_connection_error_still_sends_close -- --nocapture` | pass | 2 targeted tests passed. |
| #450 | `verify-pr-450` | `cargo test -p tentacle --features openssl-vendored cancelable_protocol_task_drops_pending_future -- --nocapture` | pass | 1 targeted test passed. |
| #453 | `verify-pr-453` | `cargo test -p tentacle --features ws,openssl-vendored websocket_accept -- --nocapture` | pass | 2 targeted WebSocket frame-limit tests passed. |
| #454 | `verify-pr-454` | `cargo test -p tentacle-secio --features openssl-vendored -- --nocapture` | pass | 42 tests passed. Initial run without vendored OpenSSL was blocked by missing `pkg-config`/OpenSSL. |
| #455 | `verify-pr-455` | `cargo test -p tokio-yamux window_update_wakes_write -- --nocapture` | pass | 2 targeted tests passed. |
| #455 | `verify-pr-455` | `cargo test -p tokio-yamux test_write_side_does_not_overwrite_read_waker -- --nocapture` | pass | 1 targeted test passed. |
| #456 | `verify-pr-456` | `cargo test -p tentacle --features openssl-vendored listen_socket_does_not_enable_reuse_address_by_default -- --nocapture` | pass | 1 targeted test passed. |
| #457 | `verify-pr-457` | `cargo test -p tentacle --features openssl-vendored --test test_peer_id -- --nocapture` | pass | 4 tests passed. |
| #458 | `verify-pr-458` | `cargo test -p tokio-yamux stream::test -- --nocapture` | pass | 12 stream tests passed. |
| #459 | `verify-pr-459` | `cargo test -p tentacle --features openssl-vendored proxy_dns_dial_sends_hostname_to_socks_server -- --nocapture` | pass | 1 targeted test passed. |
| #460 | `verify-pr-460` | `cargo test -p tentacle-multiaddr -- --nocapture` | pass | 42 integration tests, 34 quic tests, 5 doctests passed. Expected `should_panic` cases printed panic messages. |
| #461 | `verify-pr-461` | `cargo test -p tentacle --features openssl-vendored pre_shutdown_ignores_late -- --nocapture` | pass | 2 targeted tests passed. |
| #463 | `verify-pr-463` | `cargo test -p tentacle --features openssl-vendored connection_limit -- --nocapture` | pass | 2 targeted tests passed. |
| #464 | `verify-pr-464` | `cargo test -p tentacle --features openssl-vendored raw_inbound_session_registers_pending_work -- --nocapture` | pass | 1 targeted test passed. |

## Follow-up

- #443 and #445 still need review feedback resolution even though local tests pass.
- #436 and #449 should be handled next as CI-failure reproduction tasks.
- #451 should be picked up only after it leaves draft.
