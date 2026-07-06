# tentacle open PR 测试分析同步（修正版）

同步位置：nervosnetwork/acceptance-internal#1517  
分析对象：nervosnetwork/tentacle 当前 open PR，采集时间 2026-07-06  
说明：这里仅用于跟进同步。实际缺陷应提到 `nervosnetwork/tentacle`，并 reference `Xcodes-chain/tentacle-integration-tests` 中的对应 case。

上一版分组分析有偏差：它按我预设的测试主题归并，容易把结论写成“模块风险判断”。本版改为以 `nervosnetwork/tentacle` open PR 的实际状态、PR 描述、diff 文件、review/CI 状态为主。

## 状态总览

当前 open PR 共 21 个。

状态说明：

- #436：CI Test 失败，涉及 inbound handshake 连接上限。
- #449：CI Test 失败，涉及 `ServiceControl` clone 的全局 backpressure budget。
- #443：Changes requested，问题集中在 event-count backpressure 不能阻塞 close/control 事件。
- #445：Changes requested，问题集中在 yamux write-stall timeout 计时点可能导致 idle 后首次 pending 被误判超时。
- #451：Draft，作者已说明当前修复还不理想，暂不作为最终验证目标。
- #435：Approved，QUIC ServiceBuilder integration，可作为 QUIC 集成验证入口。
- #454：Approved，secio receive buffer refactor，可补充外部 partial-read 行为验证。

本轮先验证：open + non-draft + CI 成功的 PR。

纳入本轮：

- #435, #440, #443, #445, #446, #448, #450, #453, #454, #455, #456, #457, #458, #459, #460, #461, #463, #464

暂不纳入本轮：

- #436：open/non-draft，但 CI Test 当前失败，单独作为 CI failure 复现任务。
- #449：open/non-draft，但 CI Test 当前失败，单独作为 CI failure 复现任务。
- #451：draft。

## Open PR 逐条整理

| PR | 状态 | CI / review | 实际改动范围 | 测试分析 |
|---:|---|---|---|---|
| #435 | open | Approved, CI pass | `docs/quic_*`, `tentacle/examples/quic_simple.rs` | QUIC 公共使用路径验证：listen/dial、PeerId pin 成功/失败、错误客户端不能打死 listener、未配置 QUIC 时错误可诊断。 |
| #436 | open | Review required, CI Test fail | `ServiceBuilder`/`Service`/TCP transport/session/config + `test_max_connection_stalled_inbound.rs` | 核心是 inbound 半开握手也要占用 `max_connection_number`。需要复现 CI 失败并验证 stalled socket 达上限后 `max+1` 被拒绝，关闭 stalled socket 后额度释放。 |
| #440 | open | Review required, CI pass | `tentacle/src/channel/bound.rs`, `tentacle/src/substream.rs` | substream 写侧 pending 时停止继续 drain 普通 data event。重点验证 backpressure 生效，同时 close/control 事件不能被饿死。 |
| #443 | open | Changes requested, CI pass | `channel/bound.rs`, `session.rs`, `quic/session.rs` | event-count backpressure 覆盖 yamux 和 QUIC session。review 指出当前可能阻塞 `SessionClose`/`ProtocolClose`，验证应优先覆盖控制事件通行。 |
| #445 | open | Changes requested, CI pass | `yamux/src/session.rs` | 恢复 write-stall timeout。review 指出以 `last_send_success` 计时可能导致 idle 连接首次 pending 就误超时，应验证 idle 后 transient pending 不关闭，真实持续 stall 才超时。 |
| #446 | open | Review required, CI pass | `buffer.rs`, `context.rs`, `session.rs`, `quic/session.rs`, `service.rs`, `substream.rs` | 修 pending byte accounting stale counter。验证 outbound message 在 substream accept 前被 drop、missing protocol stream、session cleanup 后 pending bytes 回到 0。 |
| #448 | open | Review required, CI pass | `tentacle/src/substream.rs` | expected connection-close error 仍要触发 protocol stream cleanup。验证 `BrokenPipe`/`ConnectionReset`/`UnexpectedEof` 不上抛用户 error，但 session state 被清理。 |
| #449 | open | Review required, CI Test fail | `context.rs`, `service.rs`, `service/control.rs`, `service/event.rs`, `quic/session.rs` | 全局 service-task budget。需要先复现 CI failure，再验证多个 `ServiceControl` clone 不能放大队列容量，且 shutdown/disconnect/protocol close 仍可通过。 |
| #450 | open | Review required, CI pass | `protocol_handle_stream.rs`, `service.rs` | protocol callback future 可取消。验证 session-level 和 service-level protocol task 在 shutdown/cancel 时能 drop pending future。 |
| #451 | draft | Draft, CI pass | `tcp_base_listen.rs` | TCP protocol selection 短 peek 防 CPU spin。作者已说当前 fix 不理想，等 ready 后再纳入最终回归；当前只保留 draft 跟踪 case。 |
| #453 | open | Review required, CI pass | `service.rs`, TCP/WS transports | WebSocket handshake 应使用 `ServiceConfig::max_frame_length`。验证 tungstenite 边界拒绝超限 payload，注意 tentacle length-prefix overhead。 |
| #454 | open | Approved, CI pass | `secio/src/codec/secure_stream.rs` | `RecvBuf` 替换为 `Bytes`。验证目标不是 `Bytes::advance` 本身，而是 `SecureStream` partial read/drain 不丢、不重、不退化。 |
| #455 | open | Review required, CI pass | `yamux/src/stream.rs` | write-only yamux stream 在 send window 为 0 时应被 remote `WindowUpdate` 唤醒。验证 split/write-only 场景不依赖 reader polling。 |
| #456 | open | Review required, CI pass | `tokio_runtime/mod.rs` | 默认不再开 `SO_REUSEADDR`。验证默认 listener socket option 为 false，socket transformer opt-in 仍可开启。 |
| #457 | open | Review required, CI pass | `service.rs`, dial/kill/peer_id tests | pending `/p2p/<peer>` 不再作为 authenticated peer 去重依据。验证伪造 pending dial 不能阻止后续合法 peer dial，已认证 session 的 peer-id dedup 仍保留。 |
| #458 | open | Review required, CI pass | `yamux/src/stream.rs`, `test_session_protocol_order.rs` | `LocalClosing + FIN` 应通知 parent session close。验证 stream slot 被释放，`max_stream_count` 不被关闭 stream 占住。 |
| #459 | open | Review required, CI pass | Tokio runtime/TCP transport/proxy path | SOCKS proxy 下 `/dns4`/`/dns6` 不应本地 DNS 解析。验证 fake SOCKS5 server 收到 ATYP domain。 |
| #460 | open | Review required, CI pass | `multiaddr/src/protocol.rs`, multiaddr tests | `Protocol::P2P` raw bytes 构造路径需要校验。验证 `Multiaddr::from`/`push`/`FromIterator` 对 invalid bytes 拒绝，合法 peer id round-trip 正常。 |
| #461 | open | Review required, CI pass | `tentacle/src/service.rs` | `PreShutdown` 后 late handshake/listen completion 不能再创建 session/listener。验证 late completion 被 close/reject。 |
| #463 | open | Review required, CI pass | `tentacle/src/service.rs` | connection limit equality boundary：`active + pending == max` 即满。验证等于上限时拒绝新连接，overflow fail-closed。 |
| #464 | open | Review required, CI pass | `service.rs`, `service/helper.rs`, `session.rs` | raw inbound session handshake 要注册 pending work，避免 `forever(false)` idle shutdown 抢先退出。验证 raw inbound handshake 未完成时 service 不应 idle shutdown。 |

## 测试执行顺序建议

1. 先验证 open + non-draft + CI success 的 PR。
2. 在这组里优先跑 PR 自己声明的 targeted tests，再补外部集成 case。
3. #443/#445 虽然 CI 成功，但 review 是 changes requested，本轮只验证当前行为并记录 reviewer 指出的风险。
4. #436/#449 留到下一轮作为 CI failure 复现任务。
5. #451 等 draft ready 后再验证。

3. 外部集成测试仓库保留每个 PR 的 case 名，后续发现问题时直接在 `nervosnetwork/tentacle` issue 中引用。

## 集成测试仓库状态

仓库：`Xcodes-chain/tentacle-integration-tests`

已完成：

- Rust crate 框架，依赖本地 tentacle checkout。
- baseline case：`tcp_service_can_exchange_burst_messages`。
- #436/#463：`stalled_inbound_connections_count_toward_limit_and_release_capacity`。
- #460：`invalid_p2p_is_rejected_by_safe_constructors`。
- 其他 open PR：已放置 ignored skeleton case，作为后续 issue reference 的稳定 case name。

运行方式：

```bash
cd /Users/xue/nervosnetwork/tentacle
git fetch origin '+refs/pull/*/head:refs/remotes/origin/pr/*'
git checkout -B pr-436 origin/pr/436

cd /Users/xue/Xcodes-chain/tentacle-integration-tests
cargo test --features ws,quic -- --nocapture
cargo test stalled_inbound_connections_count_toward_limit_and_release_capacity -- --ignored --nocapture
```

## 后续 issue 规则

发现问题时，issue 提到 `nervosnetwork/tentacle`，不要提到 acceptance-internal。

建议格式：

```markdown
## Summary

<一句话说明问题>

## Affected PR / module

- PR: #<number>
- Module: <service/yamux/secio/quic/ws/etc>
- Integration case: Xcodes-chain/tentacle-integration-tests::<test_name>

## Reproduction

cd /Users/xue/nervosnetwork/tentacle
git checkout -B pr-<number> origin/pr/<number>
cd /Users/xue/Xcodes-chain/tentacle-integration-tests
cargo test <test_name> -- --ignored --nocapture

## Expected

<期望行为>

## Actual

<实际行为、日志、assertion>
```
