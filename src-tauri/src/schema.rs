// @generated automatically by Diesel CLI.

diesel::table! {
    flights (id) {
        id -> Int4,
        #[max_length = 10]
        flight_number -> Varchar,
        #[max_length = 4]
        departure_airport_code -> Varchar,
        #[max_length = 4]
        arrival_airport_code -> Varchar,
        departure_time -> Timestamp,
        user_id -> Int4,
    }
}

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

diesel::joinable!(flights -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    flights,
    users,
);
