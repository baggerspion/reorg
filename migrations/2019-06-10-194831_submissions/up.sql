CREATE TABLE IF NOT EXISTS submissions (
	id				SERIAL PRIMARY KEY,
	conference_id	INT	NOT NULL,
	user_id			INT NOT NULL,
    status_id       INT NOT NULL,
	title			VARCHAR NOT NULL,
	content			VARCHAR NOT NULL,
	tags			jsonb
)