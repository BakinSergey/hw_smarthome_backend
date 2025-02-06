use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct HomeEntity {
    pub id: Uuid,
    pub name: String,
}

impl HomeEntity {
    pub fn new(id: Uuid, name: String) -> Self {
        HomeEntity { id, name }
    }
}
