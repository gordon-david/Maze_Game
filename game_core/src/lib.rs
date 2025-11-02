use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Room {
    pub id: String,
    pub description: String,
    pub exits: Vec<Exit>,

    #[serde(default)]
    pub is_end: bool
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Exit {
    pub label: String,   // e.g. "Go through the left door"
    pub destination: String, // e.g. "middle"
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub rooms: Vec<Room>,
    pub current_room: String,
    pub is_finished: bool,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct MazeFile {
    pub rooms: Vec<Room>,
}

impl GameState {
    /// Creates a new game state with the default built-in maze
    pub fn new() -> Self {
        Self::from_rooms(Self::default_rooms())
    }

    /// Creates a new game state from the given rooms
    pub fn from_rooms(rooms: Vec<Room>) -> Self {
        if rooms.is_empty() {
            panic!("Maze must have at least one room");
        }
        
        // clone to prevent BC issue, conflicts with Self::rooms below
        let start_room: String = rooms[0].id.clone();
        
        Self {
            rooms, // "rooms" moved here
            current_room: start_room, 
            is_finished: false,
        }
    }

    /// Loads a maze from a JSON file
    pub fn load_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let maze_file: MazeFile = serde_json::from_reader(file)?;
        Ok(Self::from_rooms(maze_file.rooms))
    }

    /// Returns the default built-in maze rooms
    fn default_rooms() -> Vec<Room> {
        vec![
            Room {
                id: "start".to_string(),
                description: "You are in a small stone chamber with one door ahead.".to_string(),
                exits: vec![Exit {
                    label: "Go through the door".to_string(),
                    destination: "middle".to_string(),
                }],
                is_end: false
            },
            Room {
                id: "middle".to_string(),
                description: "You stand in a long hallway. There is a door behind and one ahead.".to_string(),
                exits: vec![
                    Exit {
                        label: "Go back".to_string(),
                        destination: "start".to_string(),
                    },
                    Exit {
                        label: "Go forward".to_string(),
                        destination: "end".to_string(),
                    },
                ],
                is_end: false
            },
            Room {
                id: "end".to_string(),
                description: "You find yourself in a bright room — the end of the maze!".to_string(),
                exits: vec![],
                is_end: true 
            },
        ]
    }

    pub fn current_room(&self) -> &Room {
        self.rooms
            .iter()
            .find(|r| r.id == self.current_room)
            .expect("current room exists")
    }

    pub fn choose_exit(&mut self, index: usize) {
        
        // First, get the destination using only immutable access
        let destination = self.current_room()
            .exits
            .get(index)
            .map(|exit| exit.destination.clone());

        let is_end = self.current_room().is_end.clone();
        
        // Now we can use the destination with mutable access
        if let Some(dest) = destination {
            self.current_room = dest;
            if is_end {
                self.is_finished = true;
            }
        }
    }
}
