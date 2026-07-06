use std::{
    io::{self, Read},
    time::Duration,
};

use tentacle_integration_tests::{
    HarnessEvent, TEST_TIMEOUT, close_stream, run_tcp_client, socket_addr, stalled_tcp_connect,
    start_tcp_server, wait_until,
};

#[test]
fn tcp_service_can_exchange_burst_messages() {
    let (addr, _server_events) = start_tcp_server(128, 256, 64);
    let client_events = run_tcp_client(addr, 0, 256, 64);

    let mut received = 0usize;
    let deadline = std::time::Instant::now() + TEST_TIMEOUT;
    while std::time::Instant::now() < deadline && received < 128 {
        if let Ok(HarnessEvent::Received(_)) = client_events.recv_timeout(Duration::from_millis(200)) {
            received += 1;
        }
    }

    assert_eq!(received, 128);
}

#[test]
#[ignore = "PR #436/#463 regression case; run explicitly on the target PR branch"]
fn stalled_inbound_connections_count_toward_limit_and_release_capacity() {
    const MAX_CONNECTIONS: usize = 2;

    let (addr, server_events) = start_tcp_server(0, 8, MAX_CONNECTIONS);
    let socket = socket_addr(&addr);

    let first = stalled_tcp_connect(socket);
    let second = stalled_tcp_connect(socket);
    std::thread::sleep(Duration::from_millis(300));

    let mut rejected = stalled_tcp_connect(socket);
    let mut buf = [0; 1];
    match rejected.read(&mut buf) {
        Ok(0) => {}
        Err(err) if err.kind() == io::ErrorKind::ConnectionReset => {}
        other => panic!("expected over-capacity stalled connection to close, got {other:?}"),
    }

    assert!(
        server_events.recv_timeout(Duration::from_millis(300)).is_err(),
        "stalled pre-handshake sockets must not create sessions"
    );

    close_stream(first);
    assert!(wait_until(Duration::from_secs(3), || {
        let mut stream = stalled_tcp_connect(socket);
        let mut buf = [0; 1];
        match stream.read(&mut buf) {
            Err(err)
                if err.kind() == io::ErrorKind::WouldBlock
                    || err.kind() == io::ErrorKind::TimedOut =>
            {
                close_stream(stream);
                true
            }
            _ => {
                close_stream(stream);
                false
            }
        }
    }));

    close_stream(second);
}
