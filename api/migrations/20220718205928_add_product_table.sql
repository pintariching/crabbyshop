CREATE TABLE product (
	id bigserial PRIMARY KEY,
	name varchar(128) NOT NULL,
	description text,
	sku varchar(128) UNIQUE,
	category_id bigint,
	inventory_id bigint,
	price decimal,
	discount_id bigint,
	created_at timestamp NOT NULL DEFAULT NOW(),
	updated_at timestamp NOT NULL DEFAULT NOW()
)