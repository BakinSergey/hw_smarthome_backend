pub trait HttpMapper<Entity, HttpObj> {
    fn to_http(entity: Entity) -> HttpObj;
    fn to_entity(http_obj: HttpObj) -> Entity;
}
