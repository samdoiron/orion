// Web UI entrypoint
// Copyright (C) 2015  Samuel Doiron, see LICENSE for details
use io::websocket_server::WebSocketServer;
use ui::windowed;
use ui::controller;

const UI_COMMUNICATION_PORT: u16 = 1742;

pub fn run() {
    let mut websocket_server = WebSocketServer::new(UI_COMMUNICATION_PORT)
        .ok().expect("Could not bind to UI port");

    let (mut ws_sender, mut command_input) = websocket_server.accept();


    // Presenter, so we can generate View Models
    let mut presenter = windowed::presenter::Presenter::new(&mut ws_sender);

    // Controller, to connect the UI to the buisness logic
    let mut controller = controller::Controller::new(
        &mut command_input,
        &mut presenter
    );

    controller.run();
}

