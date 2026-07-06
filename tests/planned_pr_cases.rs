macro_rules! planned_case {
    ($name:ident, $pr:literal, $summary:literal) => {
        #[test]
        #[ignore = $summary]
        fn $name() {
            panic!(
                "planned integration case for PR #{}: {}",
                $pr, $summary
            );
        }
    };
}

planned_case!(
    pr_435_quic_servicebuilder_listen_dial_and_peer_pin,
    435,
    "PR nervosnetwork/tentacle#435: validate QUIC listen/dial, peer-id pin success/failure, and bad handshake listener survival"
);

planned_case!(
    pr_440_443_substream_backpressure_preserves_control_events,
    440,
    "PR nervosnetwork/tentacle#440 / nervosnetwork/tentacle#443: stalled data writes must backpressure ordinary messages while close/control events still flow"
);

planned_case!(
    pr_445_yamux_write_stall_timeout_distinguishes_idle_from_blocked,
    445,
    "PR nervosnetwork/tentacle#445: idle yamux connection must not false-timeout, real write stall must timeout"
);

planned_case!(
    pr_446_pending_bytes_roll_back_after_dropped_outbound_messages,
    446,
    "PR nervosnetwork/tentacle#446: dropped outbound messages must return pending byte accounting to zero"
);

planned_case!(
    pr_448_substream_expected_close_still_cleans_session_state,
    448,
    "PR nervosnetwork/tentacle#448: expected substream close errors must still remove protocol/session state"
);

planned_case!(
    pr_449_global_service_task_budget_across_control_clones,
    449,
    "PR nervosnetwork/tentacle#449: ServiceControl clones must share one global counted-task budget"
);

planned_case!(
    pr_450_protocol_handle_tasks_are_cancellable_on_shutdown,
    450,
    "PR nervosnetwork/tentacle#450: protocol handle futures must exit when service shutdown cancels them"
);

planned_case!(
    pr_451_short_tcp_peek_does_not_spin_runtime,
    451,
    "PR nervosnetwork/tentacle#451 draft: short initial TCP input must not cause CPU spin"
);

planned_case!(
    pr_453_websocket_frame_limit_matches_service_limit_with_prefix,
    453,
    "PR nervosnetwork/tentacle#453: websocket handshake must honor service frame limit including length-prefix overhead"
);

planned_case!(
    pr_454_secio_secure_stream_drain_partial_reads,
    454,
    "PR nervosnetwork/tentacle#454: SecureStream::drain partial reads must not lose or duplicate bytes"
);

planned_case!(
    pr_455_yamux_write_only_stream_wakes_on_window_update,
    455,
    "PR nervosnetwork/tentacle#455: write-only yamux stream blocked by zero window must wake after WindowUpdate"
);

planned_case!(
    pr_456_tokio_listener_reuseaddr_disabled_by_default,
    456,
    "PR nervosnetwork/tentacle#456: default Tokio listener should not enable SO_REUSEADDR unless transformer opts in"
);

planned_case!(
    pr_457_pending_p2p_address_does_not_dedup_authenticated_peer,
    457,
    "PR nervosnetwork/tentacle#457: unauthenticated pending /p2p address must not suppress a legitimate authenticated peer dial"
);

planned_case!(
    pr_458_yamux_half_close_fin_releases_stream_slot,
    458,
    "PR nervosnetwork/tentacle#458: LocalClosing plus remote FIN must notify session and release stream slot"
);

planned_case!(
    pr_459_socks_proxy_dns_dial_uses_remote_domain_resolution,
    459,
    "PR nervosnetwork/tentacle#459: SOCKS proxied /dns4 or /dns6 dial must send ATYP domain to proxy"
);

planned_case!(
    pr_461_shutdown_ignores_late_resource_creating_events,
    461,
    "PR nervosnetwork/tentacle#461: late handshake/listen completion after PreShutdown must not create sessions or listeners"
);

planned_case!(
    pr_464_raw_inbound_handshake_blocks_idle_shutdown_until_resolved,
    464,
    "PR nervosnetwork/tentacle#464: raw inbound handshake must register pending work so idle shutdown waits for it"
);
