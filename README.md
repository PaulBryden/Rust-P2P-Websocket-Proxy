# WebSocket Example

This repository contains a simple WebSocket server implemented in Rust.

## Server

The server maintains a list of connected clients organized by room IDs. When a new client connects, the server checks if the room ID is valid and if the room is not full. If these conditions are met, the client is added to the room. When a client sends a message, the server forwards it to all other clients in the same room.

To run the server, navigate to the directory containing the server code and run `cargo run`.

## HTML Test Client Page

The client connects to the WebSocket server running on `127.0.0.1:3012`. The room ID is specified in the URL and can be changed by modifying the value of the `roomID` constant in the JavaScript code. When the connection is established, the client sends a test message to the server.

To use the client, open the HTML file in your web browser.