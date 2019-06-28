CREATE TABLE "status" (
    id      SERIAL PRIMARY KEY,
    title   VARCHAR NOT NULL 
);

INSERT INTO "status" (title)
VALUES
    ('Draft'),
    ('Submitted'),
    ('Reviewed'),
    ('Altered'),
    ('Accepted'),
    ('Rejected');