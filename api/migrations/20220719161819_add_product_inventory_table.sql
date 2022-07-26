CREATE TABLE product_inventory(
	id bigserial PRIMARY KEY,
	quantity integer NOT NULL,
	created_at timestamp NOT NULL DEFAULT NOW(),
	updated_at timestamp NOT NULL DEFAULT NOW()
)