use std::net::UdpSocket;

use dns_resolver_rust::{build_query, get_header_info, parse_resource_record};

fn main() {
    let dns_query = build_query("www.google.com");

    let dns_server_ip = "8.8.8.8:53";
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect(dns_server_ip).unwrap();
    socket.send(&dns_query.to_bytes()).unwrap();

    let mut buf = [0u8; 1024];
    let response = socket.recv(&mut buf);

    match response {
        Ok(_response_size) => {
            println!("---------------- Response Header -----------------");
            get_header_info(&buf[..12]);
            println!("---------------- Response Content -----------------");
            parse_resource_record(&buf[12..]);
        }
        Err(err) => eprintln!("ERROR {err}"),
    }
}
