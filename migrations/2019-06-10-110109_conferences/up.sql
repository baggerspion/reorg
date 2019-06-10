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
)