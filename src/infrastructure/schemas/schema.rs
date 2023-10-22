// @generated automatically by Diesel CLI.

diesel::table! {
    coffees (id) {
        id -> Nullable<Uuid>,
        #[max_length = 32]
        name -> Varchar,
        price -> Float8,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
