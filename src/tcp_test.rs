extern crate orion;
use orion::gateways::NamedDataPointGateway;
use orion::io::TcpNamedDataPointGateway;

fn main() {
    let mut gateway = TcpNamedDataPointGateway::new();
    loop {
        let point = gateway.receive();
        println!("Got point from gateway {}", point);
    }
}
