CREATE TABLE submissions (
	id				SERIAL PRIMARY KEY,
	conference_id	INT	NOT NULL,
	user_id			INT NOT NULL,
	title			VARCHAR NOT NULL,
	content			VARCHAR NOT NULL
)