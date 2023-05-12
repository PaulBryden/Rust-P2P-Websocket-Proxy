use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use ws::{listen, CloseCode, Handler, Handshake, Message, Result, Sender};

// Define a constant for the maximum number of clients allowed in a room
const MAX_CLIENTS_PER_ROOM: usize = 2;

// Define a `Server` struct to represent the WebSocket server
struct Server {
    // The `Sender` type allows sending messages to the connected client
    out: Sender,
    // The ID of the room this client is connected to
    room_id: String,
    // A shared map of room IDs to a list of connected clients
    clients: Arc<Mutex<HashMap<String, Vec<Sender>>>>,
}

// Implement the `Handler` trait for the `Server` struct
impl Handler for Server {
    // This method is called when a new client connects
    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        // Check if the URL contains a valid room ID
        if let Some(room_id) = handshake.request.resource().split('/').nth(1) {
            // Store the room ID in the `Server` struct
            self.room_id = room_id.to_string();
            // Lock the shared map of clients to prevent data races
            let mut clients = self.clients.lock().unwrap();
            // Check if the room already exists in the map
            if clients.contains_key(&self.room_id) {
                // Check if the room is full
                if clients.get(&self.room_id).unwrap().len() >= MAX_CLIENTS_PER_ROOM {
                    // If the room is full, close the connection with a `CloseCode::Policy` code
                    self.out.close(CloseCode::Policy)?;
                } else {
                    // If the room is not full, add this client to the list of connected clients for this room
                    clients.get_mut(&self.room_id).unwrap().push(self.out.clone());
                }
            } else {
                // If the room does not exist in the map yet, create it and add this client to it
                clients.insert(self.room_id.clone(), vec![self.out.clone()]);
            }
        } else {
            // If the URL does not contain a valid room ID, close the connection with a `CloseCode::Policy` code
            self.out.close(CloseCode::Policy)?;
        }
        Ok(())
    }

    // This method is called when a client sends a message
    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Lock the shared map of clients to prevent data races
        let clients = self.clients.lock().unwrap();
        // Check if this client's room exists in the map
        if let Some(room_clients) = clients.get(&self.room_id) {
            // Iterate over all clients in this room
            for client in room_clients.iter() {
                // Check if this client is not the one that sent the message
                if *client != self.out {
                    // Forward the message to this client
                    client.send(msg.clone())?;
                }
            }
        }
        Ok(())
    }

    // This method is called when a client disconnects
    fn on_close(&mut self, code: CloseCode, reason: &str) {
        // Lock the shared map of clients to prevent data races
        let mut clients = self.clients.lock().unwrap();
        // Check if this client's room exists in the map
        if let Some(room_clients) = clients.get_mut(&self.room_id) {
            // Remove this client from the list of connected clients for this room
            room_clients.retain(|client| *client != self.out);
            // Iterate over all remaining clients in this room
            for client in room_clients.iter() {
                // Disconnect this client with a `CloseCode::Normal` code
                client.close(CloseCode::Normal).unwrap();
            }
            // Remove this room from the map of rooms since it's now empty
            clients.remove(&self.room_id);
        }
    }
}

// The `main` function where execution starts
fn main() {
    // Create a new shared map of rooms and their connected clients
    let clients = Arc::new(Mutex::new(HashMap::<String, Vec<Sender>>::new()));
    // Start listening for WebSocket connections on port 3012 and handle them using instances of the `Server` struct
    listen("127.0.0.1:3012", |out| Server { out, room_id: String::new(), clients: clients.clone() }).unwrap();
}
