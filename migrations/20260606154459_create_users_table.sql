CREATE TABLE user_message (
    id SERIAL PRIMARY KEY,
    user_id BIGINT,
    us_mess VARCHAR(100) NOT NULL
);