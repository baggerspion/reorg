CREATE TABLE conferences (
	id 				SERIAL PRIMARY KEY,
	title 			VARCHAR NOT NULL,
	start_date 		TIMESTAMP NOT NULL,
	end_date 		TIMESTAMP NOT NULL,
	venue 			VARCHAR NOT NULL,
	address 		VARCHAR NOT NULL,
	city 			VARCHAR NOT NULL,
	postcode 		VARCHAR NOT NULL,
	country			VARCHAR NOT NULL,
	cfp 			VARCHAR NOT NULL
);

CREATE TABLE submissions (
	id				SERIAL PRIMARY KEY,
	conference_id	INT	NOT NULL,
	user_id			INT NOT NULL,
	title			VARCHAR NOT NULL,
	content			VARCHAR NOT NULL
);

CREATE TABLE users ( 
  	id 				SERIAL PRIMARY KEY,
  	first_name 		VARCHAR NOT NULL,
  	last_name 		VARCHAR NOT NULL,
  	email 			VARCHAR UNIQUE NOT NULL,
  	password 		VARCHAR NOT NULL
);