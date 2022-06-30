CREATE TABLE IF NOT EXISTS liturgy_settings
(
	user_id_and_liturgy		varchar PRIMARY KEY,
	prefs		json NOT NULL
);