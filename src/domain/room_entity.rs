use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RoomEntity {
    pub id: Uuid,
    pub name: String,
    pub home_id: Uuid,
}

impl RoomEntity {
    pub fn new(id: Uuid, name: String, home_id: Uuid) -> Self {
        RoomEntity { id, name, home_id }
    }
}
