use std::time::{Duration, Instant};

use tentacle::ProtocolId;
use tentacle_integration_tests::{
    HarnessEvent, run_quic_client_with_control, run_tcp_client_with_control, start_quic_server,
    start_tcp_server,
};

#[test]
#[ignore = "PR nervosnetwork/tentacle#465 regression case; run explicitly on the target PR branch"]
fn tcp_idle_session_timeout_rearms_after_protocol_close() {
    let (addr, server_events) = start_tcp_server(0, 8, 8);
    let (client_control, client_events) = run_tcp_client_with_control(addr, 0, 8, 8);

    assert_idle_session_closes_after_protocol_close(
        client_control,
        client_events,
        server_events,
    );
}

#[test]
#[ignore = "PR nervosnetwork/tentacle#465 QUIC regression case; run explicitly on the target PR branch"]
fn quic_idle_session_timeout_rearms_after_protocol_close() {
    let (addr, server_events) = start_quic_server(0, 8, 8);
    let (client_control, client_events) = run_quic_client_with_control(addr, 0, 8, 8);

    assert_idle_session_closes_after_protocol_close(
        client_control,
        client_events,
        server_events,
    );
}

fn assert_idle_session_closes_after_protocol_close(
    client_control: tentacle::service::ServiceAsyncControl,
    client_events: crossbeam_channel::Receiver<HarnessEvent>,
    server_events: crossbeam_channel::Receiver<HarnessEvent>,
) {
    let client_session_id = recv_session_open(&client_events);

    std::thread::sleep(Duration::from_millis(2500));

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        client_control
            .close_protocol(client_session_id, ProtocolId::new(1))
            .await
            .expect("close protocol");
    });

    assert!(
        recv_session_close(&client_events, &server_events, Duration::from_secs(4)),
        "session should close after the rearmed idle timeout once protocol substreams are gone"
    );
}

fn recv_session_open(events: &crossbeam_channel::Receiver<HarnessEvent>) -> tentacle::SessionId {
    let deadline = Instant::now() + Duration::from_secs(4);
    while Instant::now() < deadline {
        if let Ok(HarnessEvent::SessionOpen { session_id }) =
            events.recv_timeout(Duration::from_millis(200))
        {
            return session_id;
        }
    }
    panic!("client session did not open");
}

fn recv_session_close(
    client_events: &crossbeam_channel::Receiver<HarnessEvent>,
    server_events: &crossbeam_channel::Receiver<HarnessEvent>,
    timeout: Duration,
) -> bool {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if matches!(
            client_events.recv_timeout(Duration::from_millis(100)),
            Ok(HarnessEvent::SessionClose { .. })
        ) || matches!(
            server_events.recv_timeout(Duration::from_millis(100)),
            Ok(HarnessEvent::SessionClose { .. })
        ) {
            return true;
        }
    }
    false
}
