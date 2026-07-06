# External Case Results 2026-07-06

These results are from the external `Xcodes-chain/tentacle-integration-tests`
project running against `/Users/xue/nervosnetwork/tentacle` checked out to
selected refs.

## New Implemented Cases

| Case | Target PR | Default | Purpose |
|---|---|---|---|
| `quic_service_can_exchange_burst_messages` | nervosnetwork/tentacle#435 | active | QUIC `ServiceBuilder::quic_config` listen/dial smoke through the public service API. |
| `websocket_accepts_message_at_service_frame_limit_plus_prefix` | nervosnetwork/tentacle#453 | active | Control case: WebSocket message at `max_frame_length + 4` is accepted. |
| `websocket_rejects_message_over_service_frame_limit` | nervosnetwork/tentacle#453 | ignored | Regression case: WebSocket message at `max_frame_length + 5` is rejected by the PR. |
| `stalled_inbound_connections_count_toward_limit_and_release_capacity` | nervosnetwork/tentacle#436 | ignored | Regression case: stalled pre-handshake inbound sockets count toward `max_connection_number`. |
| `invalid_p2p_is_rejected_by_safe_constructors` | nervosnetwork/tentacle#460 | ignored | Regression case: safe multiaddr constructors reject invalid raw P2P bytes. |

Ignored cases are PR-specific because they are expected to fail on `master`
before the corresponding PR is merged.

## Results

| Ref | Command | Result | Finding |
|---|---|---|---|
| `origin/pr/435` | `cargo test --manifest-path /Users/xue/Xcodes-chain/tentacle-integration-tests/Cargo.toml --features ws,quic quic_service_can_exchange_burst_messages -- --nocapture` | pass | QUIC public service smoke works on nervosnetwork/tentacle#435. |
| `origin/pr/453` | `cargo test --manifest-path /Users/xue/Xcodes-chain/tentacle-integration-tests/Cargo.toml --features ws,quic --test ws_frame_limit -- --ignored --nocapture` | pass | Oversized WebSocket frame is rejected on nervosnetwork/tentacle#453. |
| `master` | same `websocket_rejects_message_over_service_frame_limit` case before marking ignored | fail | Master accepts the oversized WS path far enough to receive secio data, confirming the case distinguishes nervosnetwork/tentacle#453. |
| `origin/pr/436` | `cargo test --manifest-path /Users/xue/Xcodes-chain/tentacle-integration-tests/Cargo.toml --features ws,quic stalled_inbound_connections_count_toward_limit_and_release_capacity -- --ignored --nocapture` | pass | Stalled inbound sockets are counted and capacity is released on nervosnetwork/tentacle#436. |
| `master` | same stalled inbound case | fail | Master leaves the over-capacity stalled socket open; the read timed out with `WouldBlock`. |
| `origin/pr/463` | same stalled inbound case | fail | Expected: nervosnetwork/tentacle#463 does not include the nervosnetwork/tentacle#436 half-open inbound semaphore fix. The case should be scoped to nervosnetwork/tentacle#436, not nervosnetwork/tentacle#463. |
| `origin/pr/460` | `cargo test --manifest-path /Users/xue/Xcodes-chain/tentacle-integration-tests/Cargo.toml --features ws,quic invalid_p2p_is_rejected_by_safe_constructors -- --ignored --nocapture` | pass | Invalid raw P2P bytes panic as expected on nervosnetwork/tentacle#460. |
| `master` | same invalid P2P case | fail | Master did not reject invalid raw P2P bytes, confirming the case distinguishes nervosnetwork/tentacle#460. |

## Conclusion

No new defect was found in the tested PR branches from these external cases.
The useful finding is coverage confidence:

- nervosnetwork/tentacle#436, nervosnetwork/tentacle#453, and nervosnetwork/tentacle#460 now have external regression cases that fail on master and pass on the target PR.
- nervosnetwork/tentacle#435 has an external QUIC public API smoke case that passes on the target PR.
- The previous `#436 / #463` mapping was too broad; the stalled inbound case is specifically a nervosnetwork/tentacle#436 case. nervosnetwork/tentacle#463 still needs a separate equality-boundary external case.
