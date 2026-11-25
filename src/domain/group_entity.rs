use uuid::Uuid;

use crate::domain::PuzzleEntity;


pub struct GroupEntity {
    pub id: Uuid,
    pub creation_datetime: u32, // Just a timestamp
    pub all_answers_count: u32,
    pub last_puzzle_ser: PuzzleEntity,
    pub is_stopped: bool,
    pub frequency: u8, // in hours!
    pub rating: u16,
    pub next_puzzle_time: u32
}
