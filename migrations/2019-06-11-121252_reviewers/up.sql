CREATE TABLE reviewers (
    id              SERIAL PRIMARY KEY,
    conference_id   INT NOT NULL,
    user_id         INT NOT NULL
)