CREATE TABLE discount (
	id bigserial PRIMARY KEY,
	name varchar(128) NOT NULL,
	description text,
	discount_percent decimal(5, 2),
	active bool NOT NULL DEFAULT false,
	created_at timestamp NOT NULL DEFAULT NOW(),
	updated_at timestamp NOT NULL DEFAULT NOW()
)