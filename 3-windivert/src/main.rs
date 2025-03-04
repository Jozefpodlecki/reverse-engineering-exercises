use std::sync::mpsc;

use windivert::{prelude::WinDivertFlags, WinDivert};

fn main() {
    let port = 8080;
    let (tx, rx) = mpsc::channel::<Vec<u8>>();

    std::thread::spawn(move || {
        let filter = format!("tcp.SrcPort == {port}");
        let flags = WinDivertFlags::new().set_recv_only().set_sniff();
        let windivert = WinDivert::network(&filter, 0, flags).unwrap();
        let mut buffer = vec![0u8; 65535];

        loop {
            let windivert_packet = windivert.recv(Some(&mut buffer)).unwrap();
            let data = &windivert_packet.data;

            tx.send(data.to_vec()).unwrap();
        }
    });

    while let Some(data) = rx.recv().ok() {
        println!("{:?}", data);
    }
}
