CREATE TABLE IF NOT EXISTS user_settings
(
	user_id		varchar PRIMARY KEY,
    general		json NOT NULL,
	display		json NOT NULL,
	liturgy		json NOT NULL
);