CREATE TABLE IF NOT EXISTS reviews (
    id                  SERIAL PRIMARY KEY,
    reviewer_id         INT NOT NULL,
    submission_id       INT NOT NULL,
    private_comments    VARCHAR NOT NULL,
    shared_comments     VARCHAR NOT NULL,
    score               INT DEFAULT 0 NOT NULL
)