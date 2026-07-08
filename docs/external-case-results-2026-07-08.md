# External Case Results 2026-07-08

These results cover the new open, non-draft `nervosnetwork/tentacle#465`
after updating the local tentacle checkout to `origin/master` at `b1c6e5b`.

## PR Scope

- PR: nervosnetwork/tentacle#465
- Title: `Rearm idle session timeout while substreams are active`
- Files changed:
  - `tentacle/src/session.rs`
  - `tentacle/src/quic/session.rs`
- CI: all GitHub Actions checks passed.

## Implemented Case

| Case | Target PR | Default | Purpose |
|---|---|---|---|
| `idle_session_timeout_rearms_after_protocol_close` | nervosnetwork/tentacle#465 | ignored | Keep a protocol substream active through the first idle timeout tick, close the protocol, then verify the re-armed timeout closes the now protocol-less session. |

The case is ignored by default because it is expected to fail on `master`
before the PR is merged.

## Results

| Ref | Command | Result | Finding |
|---|---|---|---|
| `master` (`b1c6e5b`) | `cargo test --features ws,quic --test idle_timeout idle_session_timeout_rearms_after_protocol_close -- --ignored --nocapture` | fail | No `SessionClose` arrived after the protocol substream was closed, matching the one-shot timeout bug described by nervosnetwork/tentacle#465. |
| `refs/pull/465/head` | `cargo test --manifest-path /Users/xue/Xcodes-chain/tentacle-integration-tests/Cargo.toml --features ws,quic --test idle_timeout idle_session_timeout_rearms_after_protocol_close -- --ignored --nocapture` | pass | The timeout is re-armed and the session is closed after the protocol substream is gone. |

## Conclusion

No new defect was found in nervosnetwork/tentacle#465. The external regression
case confirms the PR behavior for yamux/TCP sessions. QUIC uses the same re-arm
logic in `tentacle/src/quic/session.rs`, but this round did not add a separate
QUIC-specific close-after-timeout reproduction.
