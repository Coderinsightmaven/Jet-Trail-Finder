// @generated automatically by Diesel CLI.

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        hashed_password -> Varchar,
        #[max_length = 255]
        username -> Varchar,
    }
}
