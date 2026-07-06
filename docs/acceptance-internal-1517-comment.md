# tentacle open PR 测试分析同步

同步位置：nervosnetwork/acceptance-internal#1517  
分析对象：nervosnetwork/tentacle 当前 open PR，采集时间 2026-07-06  
用途说明：该 issue 只用于同步跟踪，具体缺陷后续应提到 nervosnetwork/tentacle，并 reference 本集成测试仓库里的对应 case。

## 当前 open PR

共 21 个 open PR，其中 #451 是 draft。

| PR | 标题 | 状态 | 主要模块 | 测试优先级 |
|---:|---|---|---|---|
| #464 | Keep raw inbound handshakes alive during idle shutdown | open | service/session | P0 |
| #463 | Fix max connection limit off-by-one check | open | service limit | P0 |
| #461 | Ignore late resource-creating events during shutdown | open | service shutdown | P0 |
| #460 | Validate P2P bytes before serializing multiaddrs | open | multiaddr | P1 |
| #459 | Avoid local DNS resolution for SOCKS proxy dials | open | tcp/socks proxy | P1 |
| #458 | Ensure yamux streams notify session after half-close FIN | open | yamux/session cleanup | P0 |
| #457 | Deduplicate dials only by authenticated peer identity | open | dial/peer identity | P0 |
| #456 | Disable default SO_REUSEADDR on Tokio listeners | open | tokio runtime/listener | P2 |
| #455 | Wake write-only yamux streams on WindowUpdate | open | yamux flow control | P0 |
| #454 | refactor(secio): replace RecvBuf enum with Bytes | open | secio | P1 |
| #453 | Apply service frame limit to WebSocket handshakes | open | websocket/frame limit | P1 |
| #451 | fix: avoid CPU spin on short TCP peeks | draft | tcp listener | P1, draft 后再跑 |
| #450 | Restore cancellable protocol handle tasks | open | protocol task cancellation | P0 |
| #449 | Enforce global budget for service task backpressure | open | service backpressure | P0 |
| #448 | Ensure substream errors always clean up session state | open | substream cleanup | P0 |
| #446 | Fix stale pending-byte accounting on dropped outbound messages | open | pending byte accounting | P0 |
| #445 | Restore yamux write-stall timeout | open | yamux timeout | P0 |
| #443 | Backpressure small outbound messages by event count | open | channel/session/quic | P0 |
| #440 | fix: Propagate substream write backpressure | open | channel/substream | P0 |
| #436 | fix: Enforce connection limits before inbound handshakes | open | inbound connection limit | P0 |
| #435 | quic: ServiceBuilder integration | open | quic/docs/example | P1 |

## 分组分析

### 1. 连接准入、握手与关闭竞态

相关 PR：#436, #463, #464, #461, #457

- #436 把 inbound 半开握手连接计入连接上限，避免握手前连接绕过 `max_connection_number`。
- #463 修正上限判断 off-by-one：`active + pending == max` 时应视为已满。
- #464 修复 raw inbound handshake 在 idle shutdown 中被过早清理的问题。
- #461 修复进入 shutdown 后 late handshake/listen completion 又创建新资源的问题。
- #457 修复 dial 去重信任未认证 `/p2p/<peer>` 的问题，避免伪造 peer id 抑制合法 dial。

测试重点：

- 半开 TCP 连接占用连接额度，超过上限的新连接应被拒绝。
- 半开连接关闭后，额度能释放。
- 达到上限边界时不能多接受 1 个连接。
- service 进入 shutdown 后，延迟完成的 handshake/listen 不能创建 session/listener。
- pending `/p2p/<peer>` 不能作为 authenticated peer identity 去做 dial dedup。

### 2. Backpressure、控制面活性与资源计数

相关 PR：#440, #443, #446, #448, #449, #450

- #440 让 substream 写侧 backpressure 能向上游传播，不再无限 drain 到内部 buffer。
- #443 对小消息增加 event-count backpressure，同时保证高优先级 close/control 事件可通过。
- #446 修复 outbound message 被 drop 时 pending byte counter 不回滚的问题。
- #448 确保 substream expected close/error 也会清理 session state。
- #449 把 service task budget 做成跨 `ServiceControl` clone 的全局预算。
- #450 恢复 protocol handle task 的外层 cancellation，避免 shutdown 后任务泄漏。

测试重点：

- 数据队列满时普通消息应 backpressure，但 `SessionClose`/`ProtocolClose` 仍然可执行。
- 多个 `ServiceControl` clone 不能绕过全局 budget。
- substream lower sink pending 时，不应继续吸收普通数据导致内存膨胀。
- dropped outbound message、missing protocol stream、session abnormal close 后 pending bytes 回到 0。
- protocol handle future 在 shutdown/cancel 后能退出。

### 3. Yamux / secio 内部行为

相关 PR：#445, #455, #458, #454

- #445 修复 yamux write-stall timeout 的计时点，避免 idle 连接误判，同时真实 stall 要 timeout。
- #455 修复 write-only stream 没有被 WindowUpdate 唤醒的问题。
- #458 修复 half-close 后收到 FIN 不通知 session，导致 stream slot 不释放的问题。
- #454 将 secio `RecvBuf` 简化为 `Bytes`，需要覆盖 `SecureStream::drain` 的 partial-read 行为。

测试重点：

- WindowUpdate 到达后，blocked write-only stream 必须被唤醒。
- half-close + remote FIN 后 session stream count 清理，新 stream 能打开。
- idle 不误 timeout，真实持续写阻塞会 timeout。
- secio partial read/drain 不丢数据、不重复数据。

### 4. Transport、地址与协议限制

相关 PR：#460, #459, #456, #453, #451

- #460 校验 raw P2P multihash bytes，避免非法 bytes 被序列化成 multiaddr。
- #459 SOCKS proxy dial `/dns4`/`/dns6` 时应交给 proxy 做远端 DNS，不应本地解析。
- #456 默认关闭 Tokio listener 的 `SO_REUSEADDR`，需要 socket transformer opt-in。
- #453 将 service frame limit 传给 WebSocket handshake，注意 length-prefix overhead。
- #451 是 draft，目标是避免短 TCP peek 导致 CPU spin。

测试重点：

- invalid P2P bytes 构造应被拒绝，合法 peer id round-trip 正常。
- fake SOCKS5 server 观察到 ATYP domain，而不是本地解析后的 IP。
- 默认 listener `reuse_address == false`，显式 transformer 才开启。
- WebSocket payload 在配置 limit + prefix 范围内通过，超过 1 字节拒绝。
- draft 稳定后再验证短 TCP 输入不会导致 runtime spin。

### 5. QUIC ServiceBuilder 集成

相关 PR：#435

- #435 引入 QUIC ServiceBuilder 集成、文档、示例、identity/config 错误处理。

测试重点：

- TCP 和 QUIC service API 行为一致。
- `/udp/.../quic-v1` listen/dial 成功。
- peer id pin 成功/失败路径正确。
- 错误客户端 handshake 不应杀死 listener。
- `NotConfigured` / `Misconfigured` 错误可诊断。

## 集成测试仓库规划

仓库：Xcodes-chain/tentacle-integration-tests

已完成：

- Rust crate 框架，依赖本地 `../../nervosnetwork/tentacle/tentacle`。
- 基础 harness：loopback TCP service、protocol callback、event sink、timeout helper。
- baseline case：`tcp_service_can_exchange_burst_messages`。
- PR #436/#463 case：`stalled_inbound_connections_count_toward_limit_and_release_capacity`。
- PR #460 case：`invalid_p2p_is_rejected_by_safe_constructors`。
- planned case skeleton：覆盖 #435/#440/#443/#445/#446/#448/#449/#450/#451/#453/#454/#455/#456/#457/#458/#459/#461/#464。

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

```bash
cd /Users/xue/nervosnetwork/tentacle
git checkout -B pr-<number> origin/pr/<number>
cd /Users/xue/Xcodes-chain/tentacle-integration-tests
cargo test <test_name> -- --ignored --nocapture
```

## Expected

<期望行为>

## Actual

<实际行为、日志、assertion>
```
