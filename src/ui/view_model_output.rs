// Copyright (C) 2015  Samuel Doiron, see LICENSE for details
use io::serialize::Serialize;
use io::WriteTransport;

pub trait ViewModelOutput<T> {
    fn send_vm(&mut self, &T);
}

impl<'a, V, T> ViewModelOutput<V> for T
    where V: Serialize,
          T: WriteTransport {

    fn send_vm(&mut self, vm: &V) {
        self.send(&vm.serialize())
            .ok().expect("failed to send vm on write transport");
    }
}
