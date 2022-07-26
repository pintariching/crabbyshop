CREATE TABLE category (
	id bigserial PRIMARY KEY,
	name varchar(128) NOT NULL,
	parent_id bigint
)