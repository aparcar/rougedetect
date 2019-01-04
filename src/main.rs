extern crate dhcp4r;
extern crate time;
extern crate rand;

//use std::net::{UdpSocket};
use dhcp4r::{packet};
//use packet::*;
//use std::mem::size_of;
use rand::Rng;

pub struct DhcpOffer {
    pub server_address: [u8; 4],
    pub offered_address: [u8; 4],
    pub lease_time: u32,
    pub renewal_time: u32,
    pub rebinding_time: u32,
}

fn main() -> std::io::Result<()> {
    //server::Server::serve(UdpSocket::bind("0.0.0.0:67").unwrap(), [0,0,0,0], MyServer{});
    let mut out_buf: [u8; 272] = [0; 272];
    let mut opts = Vec::with_capacity(0);

    println!("0: {} \n1: {}", out_buf[0], out_buf[1]);

    println!("opts: {}", opts.len());
    println!("buf: {}", out_buf.len());

    let mut rng = rand::thread_rng();
    //    let offer_size = size_of::<DhcpOffer>();
    let dhcp_request = packet::Packet {
        reply: false,
        hops: 0,
        xid: [rng.gen(), rng.gen(), rng.gen(), rng.gen()],
        secs: 0,
        broadcast: true,
        ciaddr: [0, 0, 0, 0],
        yiaddr: [0, 0, 0, 0],
        siaddr: [0, 0, 0, 0],
        giaddr: [0, 0, 0, 0],
        chaddr: [0, 0, 0, 0, 0, 0],
        options: opts,
    };
    let foobar = dhcp_request.encode(&mut out_buf);
    //let socket = UdpSocket::bind("0.0.0.0:67")?;
    //socket.send_to(,
    //"255.255.255.255:68").expect("couldn't send data");

    //        let mut socket = UdpSocket::bind("0.0.0.0:67")?;

    //       socket.send_to(buf, &src)?;
    // Receives a single datagram message on the socket. If `buf` is too small to hold
    // the message, it will be cut off.
    //      let mut buf = [0; offer_size];
    //    let (amt, src) = socket.recv_from(&mut offer_size)?;

    // Redeclare `buf` as slice of the received data and send reverse data back to origin.
    //     let buf = &mut buf[..amt];
    Ok(())
}
