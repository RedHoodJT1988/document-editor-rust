use std::collections::HashMap;

pub type RoomId = String;

pub struct RoomData {
  pub rooms: HashMap<RoomId, Vec<Addr<WebSocket>>>,
  pub client_rooms: HashMap<Addr<WebSocket>, RoomId>,
}

impl RoomData {
  pub fn new() -> Self {
    RoomData {
      rooms: HashMap::new(),
      client_rooms: HashMap::new()
    }
  }
}

// let room_data = RoomData::new();