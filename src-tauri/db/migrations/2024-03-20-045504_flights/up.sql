-- Your SQL goes here
CREATE TABLE flights (
    id SERIAL PRIMARY KEY,
    flight_number VARCHAR(10) NOT NULL,
    departure_airport_code VARCHAR(4) NOT NULL,
    arrival_airport_code VARCHAR(4) NOT NULL,
    departure_time TIMESTAMP NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (user_id) -- Adjusted to reference user_id
);