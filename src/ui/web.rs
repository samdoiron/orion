// Web UI entrypoint
// Copyright (C) 2015  Samuel Doiron, see LICENSE for details
use io::websocket_server::WebSocketServer;
use ui::windowed;
use ui::controller;
use ui::debug_view_model_output::DebugViewModelOutput;
use ui::transport_command_input::TransportCommandInput;

const UI_COMMUNICATION_PORT: u16 = 1742;

pub fn run() {
    let mut websocket_server = WebSocketServer::new(UI_COMMUNICATION_PORT)
        .ok().expect("Could not bind to UI port");

    let (_, mut ws_receiver) = websocket_server.accept();

    // Command input, so we can receive UI commands
    let mut command_input = TransportCommandInput::new(&mut ws_receiver);

    // View Model output, so we can give ViewModels to the UI
    let mut vm_output = DebugViewModelOutput::new();

    // Presenter, so we can generate View Models
    let mut presenter = windowed::presenter::Presenter::new(&mut vm_output);

    // Controller, to connect the UI to the buisness logic
    let mut controller = controller::Controller::new(
        &mut command_input,
        &mut presenter
    );

    controller.run();
}

