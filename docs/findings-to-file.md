# Findings To File In `nervosnetwork/tentacle`

The current GitHub integration cannot create issues in `nervosnetwork/tentacle`
(`Resource not accessible by integration`). File these once a user-authenticated
GitHub session is available.

## 1. Add integration coverage for secio SecureStream partial reads after RecvBuf refactor

Suggested repository: `nervosnetwork/tentacle`

Related PR: https://github.com/nervosnetwork/tentacle/pull/454

### Summary

PR #454 replaces the `RecvBuf` enum in `tentacle-secio` with `bytes::Bytes`.
The current PR tests exercise `Bytes::advance()` itself, but they do not verify
the changed `SecureStream::drain` / `recv_buf` partial-read behavior.

### Suggested Coverage

Add a regression case that forces encrypted data to be decoded into
`SecureStream.recv_buf`, then reads it through several small `ReadBuf`s and
asserts:

- bytes are returned in order
- repeated small reads advance the internal buffer correctly
- the internal buffer empties after the final drain
- both in-place and non-in-place cipher paths are covered if practical

### Integration Project Reference

- Project: `Xcodes-chain/tentacle-integration-tests`
- Local path: `/Users/xue/Xcodes-chain/tentacle-integration-tests`
- Plan file: `docs/test-plan.md`
- Matrix row: `Yamux and secio internals / #454`

## 2. Add external WebSocket frame-limit integration case

Suggested repository: `nervosnetwork/tentacle`

Related PR: https://github.com/nervosnetwork/tentacle/pull/453

### Summary

PR #453 applies `ServiceConfig::max_frame_length` to WebSocket handshakes.
Review comments identified that the default length-delimited framing overhead
must be accounted for and that tests should avoid duplicating the framing
overhead constant.

### Suggested Coverage

Add an external service-level WebSocket test that:

- configures a small `max_frame_length`
- sends a message at the accepted wire size
- sends a message one byte over the accepted wire size
- asserts the accepted/rejected behavior at the WebSocket parser boundary

### Integration Project Reference

- Project: `Xcodes-chain/tentacle-integration-tests`
- Planned case: `websocket_accepts_limit_plus_prefix_and_rejects_plus_one`
- Plan file: `docs/test-plan.md`

## 3. Hold draft CPU-spin validation until PR #451 stabilizes

Suggested repository: `nervosnetwork/tentacle`

Related PR: https://github.com/nervosnetwork/tentacle/pull/451

### Summary

PR #451 is still draft, and the author noted that the current fix is not ideal
and is being improved. Keep the external integration case planned, but avoid
treating it as a merge-blocking acceptance test until the PR is updated.

### Suggested Coverage

After the PR stabilizes, add an external test that sends short initial TCP input
and confirms the runtime remains schedulable while the protocol selection path
waits or times out.

### Integration Project Reference

- Project: `Xcodes-chain/tentacle-integration-tests`
- Planned case: `short_initial_tcp_input_does_not_starve_runtime`
- Plan file: `docs/test-plan.md`

