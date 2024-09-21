use std::net::UdpSocket;

use dns_resolver_rust::header::DnsQuery;

fn main() {
    let dns_query = DnsQuery::build_query("www.google.com");

    let dns_server_ip = "8.8.8.8:53";
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect(dns_server_ip).unwrap();
    socket.send(&dns_query.to_bytes()).unwrap();

    let mut buf = [0u8; 1024];
    let response = socket.recv(&mut buf);

    match response {
        Ok(response_size) => {
            println!("Received {response_size} bytes");
            println!("Content {:?}", &buf[..response_size]);
        }
        Err(err) => eprintln!("ERROR {err}"),
    }
}
