extern crate orion;
extern crate time;

use orion::gateways::NamedDataPointGateway;
use orion::io::TcpNamedDataPointGateway;

const SECOND: u64 = 1_000_000_000;

fn main() {
    let mut gateway = TcpNamedDataPointGateway::new();
    let mut last_report = time::precise_time_ns();
    let mut num_received = 0;
    loop {
        let _ = gateway.receive();
        num_received += 1;
        let now = time::precise_time_ns();
        let delta = now - last_report;
        if delta > SECOND {
            last_report = now;
            println!("Transfering at {} packages/s", SECOND as f64 * (num_received as f64 / delta as f64));
            num_received = 0;
        }
    }
}
