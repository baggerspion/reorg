CREATE TABLE IF NOT EXISTS reviewers (
    id              SERIAL PRIMARY KEY,
    conference_id   INT NOT NULL,
    user_id         INT NOT NULL
)