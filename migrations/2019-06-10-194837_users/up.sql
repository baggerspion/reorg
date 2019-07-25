CREATE TABLE IF NOT EXISTS users ( 
  	id 				SERIAL PRIMARY KEY,
  	first_name 		VARCHAR NOT NULL,
  	last_name 		VARCHAR NOT NULL,
  	email 			VARCHAR UNIQUE NOT NULL,
  	password 		VARCHAR NOT NULL,
	twitter			VARCHAR,
	website			VARCHAR,
	roles			VARCHAR[]
)