// @generated automatically by Diesel CLI.

diesel::table! {
    coffees (id) {
        id -> Int8,
        #[max_length = 32]
        name -> Varchar,
        price -> Float8,
    }
}
