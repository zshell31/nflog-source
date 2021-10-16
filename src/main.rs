use std::io;
use tokio::sync::mpsc;
use tokio_nflog::{AddressFamily, CopyMode, Flags, Message, MessageHandler, QueueConfig};

mod hwheader;
mod msg;
mod packet;

use msg::NflogMessage;

struct Handler {
    tx: mpsc::Sender<NflogMessage>,
}

impl Handler {
    fn new() -> (Self, mpsc::Receiver<NflogMessage>) {
        let (tx, rx) = mpsc::channel(1000000);
        (Self { tx }, rx)
    }
}

impl MessageHandler for Handler {
    fn handle(&mut self, msg: Message<'_>) {
        let msg = NflogMessage::new(msg);
        if let Err(e) = self.tx.try_send(msg) {
            println!("Sending error: {}", e);
        }
    }
}

async fn listen_queue(config: QueueConfig, handler: Handler) -> io::Result<()> {
    let queue = config.build(handler)?;

    println!("Starting nflog listening");
    let mut socket = queue.socket()?;
    socket.listen().await
}

async fn run() -> io::Result<()> {
    let config = QueueConfig {
        address_families: vec![AddressFamily::Inet],
        group_num: 12,
        copy_mode: Some(CopyMode::Packet),
        range: Some(0xffff),
        flags: Some(Flags::SEQUENCE),
        ..Default::default()
    };
    let (handler, mut rx) = Handler::new();
    tokio::spawn(listen_queue(config, handler));

    while let Some(msg) = rx.recv().await {
        println!("{}", serde_json::to_string_pretty(&msg).unwrap());
    }

    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    run().await
}
