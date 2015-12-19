// Web UI entrypoint
// Copyright (C) 2015  Samuel Doiron
use io::websocket_server::WebSocketServer;
use transport::{ReadTransport, WriteTransport};
use ui::windowed;
use ui::presenter;
use ui::controller;
use ui::debug_view_model_output::DebugViewModelOutput;
use ui::transport_command_input::TransportCommandInput;

const port: u16 = 1742;

pub fn run() {
    let mut websocket_server = WebSocketServer::new(port)
        .ok().expect("Could not bind to port");

    let (_, mut ws_receiver) = websocket_server.accept();
    let mut command_input = TransportCommandInput::new(&mut ws_receiver);
    let mut vm_output = DebugViewModelOutput::new();
    let mut presenter = windowed::presenter::Presenter::new(&mut vm_output);
    let mut controller = controller::Controller::new(
        &mut command_input,
        &mut presenter
    );

    controller.run();
}

