// @generated automatically by Diesel CLI.

diesel::table! {
    devices (id) {
        id -> Uuid,
        name -> Varchar,
        data -> Json,
        room_id -> Uuid,
    }
}

diesel::table! {
    homes (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

diesel::table! {
    reports (id) {
        id -> Uuid,
        home_id -> Uuid,
        query -> Json,
        content -> Json,
        created -> Timestamptz,
    }
}

diesel::table! {
    rooms (id) {
        id -> Uuid,
        name -> Varchar,
        home_id -> Uuid,
    }
}

diesel::joinable!(devices -> rooms (room_id));
diesel::joinable!(reports -> homes (home_id));
diesel::joinable!(rooms -> homes (home_id));

diesel::allow_tables_to_appear_in_same_query!(devices, homes, reports, rooms,);
