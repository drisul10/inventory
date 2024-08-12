// @generated automatically by Diesel CLI.

use diesel::prelude::*;

table! {
    use diesel::sql_types::*;
    use diesel::sql_types::Uuid;

    items (id) {
        id -> Uuid,
        sequence_id -> Int8,
        name -> Varchar,
        unit -> Varchar,
        stock -> Float8,
        rack -> Nullable<Varchar>,
        location -> Nullable<Varchar>,
        is_deleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}