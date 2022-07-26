ALTER TABLE product
	ADD CONSTRAINT product_category_fk FOREIGN KEY (category_id) 
	REFERENCES category(id) 
		ON DELETE SET NULL
		ON UPDATE CASCADE;

ALTER TABLE product
	ADD CONSTRAINT product_inventory_fk FOREIGN KEY (inventory_id)
	REFERENCES product_inventory(id)
		ON DELETE SET NULL
		ON UPDATE CASCADE;

ALTER TABLE product
	ADD CONSTRAINT product_discount_fk FOREIGN KEY (discount_id)
	REFERENCES discount(id)
		ON DELETE SET NULL
		ON UPDATE CASCADE;

ALTER TABLE category
	ADD CONSTRAINT category_parent_fk FOREIGN KEY (parent_id)
	REFERENCES category(id)
		ON DELETE CASCADE
		ON UPDATE CASCADE;