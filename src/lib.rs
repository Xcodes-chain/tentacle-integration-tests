use std::{
    net::{IpAddr, Shutdown, SocketAddr, TcpStream as StdTcpStream},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
    time::{Duration, Instant},
};

use bytes::Bytes;
use futures::channel;
use tentacle::{
    ProtocolId, async_trait,
    builder::{MetaBuilder, ServiceBuilder},
    context::{ProtocolContext, ProtocolContextMutRef, ServiceContext},
    multiaddr::{Multiaddr, Protocol},
    quic::config::QuicConfig,
    secio::SecioKeyPair,
    service::{ProtocolHandle, ProtocolMeta, Service, ServiceEvent, TargetProtocol},
    traits::{ServiceHandle, ServiceProtocol},
};

pub const TEST_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Clone, Debug)]
pub enum HarnessEvent {
    SessionOpen,
    SessionClose,
    Received(Vec<u8>),
    ServiceError(String),
}

#[derive(Clone)]
pub struct EventSink {
    sender: crossbeam_channel::Sender<HarnessEvent>,
}

impl EventSink {
    pub fn new() -> (Self, crossbeam_channel::Receiver<HarnessEvent>) {
        let (sender, receiver) = crossbeam_channel::unbounded();
        (Self { sender }, receiver)
    }

    pub fn send(&self, event: HarnessEvent) {
        let _ = self.sender.send(event);
    }
}

pub struct EchoProtocol {
    sink: EventSink,
    inbound_burst: usize,
    received_count: Arc<AtomicUsize>,
}

impl EchoProtocol {
    pub fn new(sink: EventSink, inbound_burst: usize, received_count: Arc<AtomicUsize>) -> Self {
        Self {
            sink,
            inbound_burst,
            received_count,
        }
    }
}

#[async_trait]
impl ServiceProtocol for EchoProtocol {
    async fn init(&mut self, _context: &mut ProtocolContext) {}

    async fn connected(&mut self, context: ProtocolContextMutRef<'_>, _version: &str) {
        if context.session.ty.is_inbound() {
            for i in 0..self.inbound_burst {
                let _ = context
                    .send_message(Bytes::from(format!("burst-{i:04}")))
                    .await;
            }
        }
    }

    async fn received(&mut self, context: ProtocolContextMutRef<'_>, data: Bytes) {
        self.received_count.fetch_add(1, Ordering::SeqCst);
        self.sink.send(HarnessEvent::Received(data.to_vec()));
        if data.as_ref() == b"shutdown" {
            let _ = context.shutdown().await;
        }
    }
}

pub struct HarnessHandle {
    sink: EventSink,
}

impl HarnessHandle {
    pub fn new(sink: EventSink) -> Self {
        Self { sink }
    }
}

#[async_trait]
impl ServiceHandle for HarnessHandle {
    async fn handle_error(
        &mut self,
        _context: &mut ServiceContext,
        error: tentacle::service::ServiceError,
    ) {
        self.sink.send(HarnessEvent::ServiceError(format!("{error:?}")));
    }

    async fn handle_event(&mut self, _context: &mut ServiceContext, event: ServiceEvent) {
        match event {
            ServiceEvent::SessionOpen { .. } => self.sink.send(HarnessEvent::SessionOpen),
            ServiceEvent::SessionClose { .. } => self.sink.send(HarnessEvent::SessionClose),
            _ => {}
        }
    }
}

pub fn protocol_meta(
    id: ProtocolId,
    sink: EventSink,
    inbound_burst: usize,
    received_count: Arc<AtomicUsize>,
) -> ProtocolMeta {
    MetaBuilder::new()
        .id(id)
        .service_handle(move || {
            ProtocolHandle::Callback(Box::new(EchoProtocol::new(
                sink.clone(),
                inbound_burst,
                received_count.clone(),
            )))
        })
        .build()
}

pub fn build_service(
    sink: EventSink,
    inbound_burst: usize,
    channel_size: usize,
    max_connections: usize,
) -> Service<HarnessHandle, SecioKeyPair> {
    build_service_with_options(sink, inbound_burst, channel_size, max_connections, None, false)
}

pub fn build_service_with_options(
    sink: EventSink,
    inbound_burst: usize,
    channel_size: usize,
    max_connections: usize,
    max_frame_length: Option<usize>,
    enable_quic: bool,
) -> Service<HarnessHandle, SecioKeyPair> {
    let received_count = Arc::new(AtomicUsize::new(0));
    let mut builder = ServiceBuilder::default()
        .insert_protocol(protocol_meta(
            ProtocolId::new(1),
            sink.clone(),
            inbound_burst,
            received_count,
        ))
        .handshake_type(SecioKeyPair::secp256k1_generated().into())
        .max_connection_number(max_connections)
        .set_channel_size(channel_size)
        .timeout(Duration::from_secs(2));

    if let Some(max_frame_length) = max_frame_length {
        builder = builder.max_frame_length(max_frame_length);
    }

    if enable_quic {
        builder = builder.quic_config(QuicConfig::default());
    }

    builder.build(HarnessHandle::new(sink))
}

pub fn start_tcp_server(
    inbound_burst: usize,
    channel_size: usize,
    max_connections: usize,
) -> (Multiaddr, crossbeam_channel::Receiver<HarnessEvent>) {
    let (sink, receiver) = EventSink::new();
    let (addr_sender, addr_receiver) = channel::oneshot::channel::<Multiaddr>();
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let mut service = build_service(sink, inbound_burst, channel_size, max_connections);
        rt.block_on(async move {
            let listen_addr = service
                .listen("/ip4/127.0.0.1/tcp/0".parse().unwrap())
                .await
                .unwrap();
            addr_sender.send(listen_addr).unwrap();
            service.run().await
        });
    });
    let listen_addr = futures::executor::block_on(addr_receiver).unwrap();
    (listen_addr, receiver)
}

pub fn start_ws_server_with_frame_limit(
    inbound_burst: usize,
    channel_size: usize,
    max_connections: usize,
    max_frame_length: usize,
) -> (Multiaddr, crossbeam_channel::Receiver<HarnessEvent>) {
    let (sink, receiver) = EventSink::new();
    let (addr_sender, addr_receiver) = channel::oneshot::channel::<Multiaddr>();
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let mut service = build_service_with_options(
            sink,
            inbound_burst,
            channel_size,
            max_connections,
            Some(max_frame_length),
            false,
        );
        rt.block_on(async move {
            let listen_addr = service
                .listen("/ip4/127.0.0.1/tcp/0/ws".parse().unwrap())
                .await
                .unwrap();
            addr_sender.send(listen_addr).unwrap();
            service.run().await
        });
    });
    let listen_addr = futures::executor::block_on(addr_receiver).unwrap();
    (listen_addr, receiver)
}

pub fn start_quic_server(
    inbound_burst: usize,
    channel_size: usize,
    max_connections: usize,
) -> (Multiaddr, crossbeam_channel::Receiver<HarnessEvent>) {
    let (sink, receiver) = EventSink::new();
    let (addr_sender, addr_receiver) = channel::oneshot::channel::<Multiaddr>();
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let mut service = build_service_with_options(
            sink,
            inbound_burst,
            channel_size,
            max_connections,
            None,
            true,
        );
        rt.block_on(async move {
            let listen_addr = service
                .listen("/ip4/127.0.0.1/udp/0/quic-v1".parse().unwrap())
                .await
                .unwrap();
            addr_sender.send(listen_addr).unwrap();
            service.run().await
        });
    });
    let listen_addr = futures::executor::block_on(addr_receiver).unwrap();
    (listen_addr, receiver)
}

pub fn run_quic_client(
    addr: Multiaddr,
    inbound_burst: usize,
    channel_size: usize,
    max_connections: usize,
) -> crossbeam_channel::Receiver<HarnessEvent> {
    let (sink, receiver) = EventSink::new();
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let mut service = build_service_with_options(
            sink,
            inbound_burst,
            channel_size,
            max_connections,
            None,
            true,
        );
        rt.block_on(async move {
            service.dial(addr, TargetProtocol::All).await.unwrap();
            service.run().await
        });
    });
    receiver
}

pub fn run_tcp_client(
    addr: Multiaddr,
    inbound_burst: usize,
    channel_size: usize,
    max_connections: usize,
) -> crossbeam_channel::Receiver<HarnessEvent> {
    let (sink, receiver) = EventSink::new();
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let mut service = build_service(sink, inbound_burst, channel_size, max_connections);
        rt.block_on(async move {
            service.dial(addr, TargetProtocol::All).await.unwrap();
            service.run().await
        });
    });
    receiver
}

pub fn socket_addr(listen_addr: &Multiaddr) -> SocketAddr {
    let mut ip = None;
    let mut port = None;
    for proto in listen_addr.iter() {
        match proto {
            Protocol::Ip4(addr) => ip = Some(IpAddr::V4(addr)),
            Protocol::Ip6(addr) => ip = Some(IpAddr::V6(addr)),
            Protocol::Tcp(p) => port = Some(p),
            _ => {}
        }
    }
    SocketAddr::new(ip.expect("ip component"), port.expect("tcp component"))
}

pub fn ws_url(listen_addr: &Multiaddr) -> String {
    let socket = socket_addr(listen_addr);
    format!("ws://{socket}")
}

pub fn stalled_tcp_connect(addr: SocketAddr) -> StdTcpStream {
    let stream = StdTcpStream::connect(addr).unwrap();
    stream.set_read_timeout(Some(Duration::from_millis(800))).unwrap();
    stream
}

pub fn close_stream(stream: StdTcpStream) {
    let _ = stream.shutdown(Shutdown::Both);
    drop(stream);
}

pub fn wait_until<F>(timeout: Duration, mut condition: F) -> bool
where
    F: FnMut() -> bool,
{
    let start = Instant::now();
    while start.elapsed() < timeout {
        if condition() {
            return true;
        }
        thread::sleep(Duration::from_millis(20));
    }
    condition()
}
