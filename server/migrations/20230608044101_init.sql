CREATE TABLE IF NOT EXISTS accounts (
	name TEXT PRIMARY KEY, 
	privkey TEXT, 
	pubkey TEXT, 
	webfinger TEXT, 
	actor TEXT, 
	apikey TEXT, 
	followers TEXT, 
	messages TEXT
);

CREATE TABLE IF NOT EXISTS messages (
	guid TEXT PRIMARY KEY, 
	message TEXT
);
