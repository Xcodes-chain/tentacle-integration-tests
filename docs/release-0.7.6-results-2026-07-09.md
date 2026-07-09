# Tentacle 0.7.6 Release Validation 2026-07-09

Release: https://github.com/nervosnetwork/tentacle/releases/tag/0.7.6

Local tentacle checkout:

- Tag: `0.7.6`
- Commit: `2ec9bc619983b444233b9b2f1e1bb3eae44dd657`

## Release Scope

`0.7.6` is a normal release, published on 2026-07-09. Compared with
`0.7.5`, it is ahead by 17 commits.

Release notes include:

- QUIC multiaddr parsing and QUIC endpoint/session implementation:
  - https://github.com/nervosnetwork/tentacle/pull/430
  - https://github.com/nervosnetwork/tentacle/pull/431
  - https://github.com/nervosnetwork/tentacle/pull/432
  - https://github.com/nervosnetwork/tentacle/pull/433
- Empty address error handling:
  - https://github.com/nervosnetwork/tentacle/pull/427
- wasm X25519 peer key length validation:
  - https://github.com/nervosnetwork/tentacle/pull/442
- Yamux/substream/runtime/secio fixes:
  - https://github.com/nervosnetwork/tentacle/pull/437
  - https://github.com/nervosnetwork/tentacle/pull/439
  - https://github.com/nervosnetwork/tentacle/pull/441
  - https://github.com/nervosnetwork/tentacle/pull/447
  - https://github.com/nervosnetwork/tentacle/pull/454
  - https://github.com/nervosnetwork/tentacle/pull/458

## External Integration Results

Command:

```bash
cd /Users/xue/nervosnetwork/tentacle
git checkout 0.7.6

cargo test --manifest-path /Users/xue/Xcodes-chain/tentacle-integration-tests/Cargo.toml --features ws,quic -- --nocapture
```

Result: pass.

Coverage:

- TCP + secio + yamux baseline message delivery.
- QUIC public ServiceBuilder listen/dial smoke.
- WebSocket at-limit control case.
- Valid P2P multiaddr round-trip.

Ignored PR-specific regression cases were not run by default because they
target open PR branches rather than this release tag.

## Upstream Package Results

| Command | Result | Notes |
|---|---|---|
| `cargo test -p tentacle --features quic,ws,tls,unstable,openssl-vendored -- --nocapture` | pass | 110 lib tests plus integration tests and doctests passed. Covers QUIC parsing/verifier/e2e, TCP/WS/TLS paths, proxy protocol, protocol open/close, session protocol order, and runtime compatibility. |
| `cargo test -p tentacle-secio --features openssl-vendored -- --nocapture` | pass | 41 tests passed. Expected `should_panic` logs were printed. One test-only unused import warning was observed in `secio/src/codec/secure_stream.rs`. |
| `cargo test -p tokio-yamux -- --nocapture` | pass | 23 tests passed, including `test_remote_fin_after_local_close_notifies_session`. |
| `cargo test -p tentacle-multiaddr -- --nocapture` | pass | 38 integration tests, 2 onion tests, 34 QUIC tests, and 5 doctests passed. |

## Conclusion

No release-blocking issue was found for `nervosnetwork/tentacle@0.7.6` in this
round.

The release-specific high-risk areas from the notes were covered by a
combination of external integration tests and upstream package tests:

- QUIC address parsing, certificate/verifier behavior, endpoint/session e2e,
  and public listen/dial smoke passed.
- Secio receive-buffer refactor passed upstream tests, including partial read
  coverage.
- Yamux half-close FIN notification passed upstream tests.
- Multiaddr UDP/QUIC parsing and round-trip compatibility passed.

Residual notes:

- The external integration framework's open-PR regression cases remain ignored
  by default and are not release blockers.
- The observed `tentacle-secio` unused import warning is not a functional
  failure, but can be cleaned up separately.
