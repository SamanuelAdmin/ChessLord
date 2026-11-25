use uuid::Uuid;

use crate::domain::PuzzleEntity;


pub struct GroupEntity {
    id: Uuid,
    creation_datetime: u32, // Just a timestamp
    all_answers_count: u32,
    last_puzzle_ser: PuzzleEntity,
    is_stopped: bool,
    frequency: u8, // in hours!
    rating: u16,
    next_puzzle_time: u32
}
