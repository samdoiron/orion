// Copyright (C) 2015  Samuel Doiron
use io::serialize::Serialize;
use io::websocket_server::WebSocketServer;
use transport::WriteTransport;

pub trait ViewModelOutput<T> {
    fn send_vm(&mut self, &T);
}

impl<'a, T> ViewModelOutput<T> for WriteTransport
    where T: Serialize {

    fn send_vm(&mut self, vm: &T) {
        self.send(&vm.serialize())
            .ok().expect("failed to send vm on write transport");
    }
}
