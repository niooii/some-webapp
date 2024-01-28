CREATE TABLE messages_to_world (
    id BIGSERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    time_created BIGINT NOT NULL
);