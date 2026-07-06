use std::time::Duration;

use futures::{SinkExt, StreamExt};
use tentacle_integration_tests::{start_ws_server_with_frame_limit, ws_url};
use tokio_tungstenite::{connect_async, tungstenite::Message};

const LENGTH_PREFIX_LEN: usize = 4;

#[test]
#[ignore = "PR nervosnetwork/tentacle#453 regression case; run explicitly on the target PR branch"]
fn websocket_rejects_message_over_service_frame_limit() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let max_frame_length = 1024 * 1024;
        let (addr, _events) = start_ws_server_with_frame_limit(0, 8, 8, max_frame_length);
        let url = ws_url(&addr);
        let (mut ws, _) = connect_async(&url).await.expect("connect websocket");

        if ws
            .send(Message::Binary(vec![0; max_frame_length + LENGTH_PREFIX_LEN + 1].into()))
            .await
            .is_err()
        {
            return;
        }

        let closed = tokio::time::timeout(Duration::from_secs(3), ws.next())
            .await
            .expect("oversized websocket frame should get a close/error promptly");

        assert!(
            matches!(closed, Some(Err(_)) | Some(Ok(Message::Close(_))) | None),
            "expected close/error for oversized websocket message, got {closed:?}"
        );
    });
}

#[test]
fn websocket_accepts_message_at_service_frame_limit_plus_prefix() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let max_frame_length = 1024 * 1024;
        let (addr, _events) = start_ws_server_with_frame_limit(0, 8, 8, max_frame_length);
        let url = ws_url(&addr);
        let (mut ws, _) = connect_async(&url).await.expect("connect websocket");

        ws.send(Message::Binary(vec![0; max_frame_length + LENGTH_PREFIX_LEN].into()))
            .await
            .expect("message at service frame limit plus prefix should be accepted by websocket");
    });
}
