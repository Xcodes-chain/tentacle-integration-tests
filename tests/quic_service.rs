use std::time::Duration;

use tentacle_integration_tests::{HarnessEvent, TEST_TIMEOUT, run_quic_client, start_quic_server};

#[test]
fn quic_service_can_exchange_burst_messages() {
    let (addr, _server_events) = start_quic_server(64, 256, 64);
    let client_events = run_quic_client(addr, 0, 256, 64);

    let mut received = 0usize;
    let deadline = std::time::Instant::now() + TEST_TIMEOUT;
    while std::time::Instant::now() < deadline && received < 64 {
        if let Ok(HarnessEvent::Received(_)) =
            client_events.recv_timeout(Duration::from_millis(200))
        {
            received += 1;
        }
    }

    assert_eq!(received, 64);
}
