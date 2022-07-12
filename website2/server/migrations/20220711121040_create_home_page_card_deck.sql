CREATE TABLE IF NOT EXISTS home_page
(
	user_id		varchar PRIMARY KEY,
	deck		json NOT NULL
);

CREATE TABLE IF NOT EXISTS favorites
(
	id			bigserial PRIMARY KEY,
	user_id		varchar,
	content		json NOT NULL
);

CREATE TABLE IF NOT EXISTS bookmarks
(
	id			bigserial PRIMARY KEY,
	user_id		varchar,
	url			text NOT NULL,
	label		text NOT NULL,
	preview		text NOT NULL
);