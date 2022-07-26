CREATE VIEW product_flat_view AS 
SELECT
	p.id as "id!", p.name as "name!", p.description, p.sku, p.price,
	p.category_id, p.inventory_id, p.discount_id,
	p.created_at as "created_at!", p.updated_at as "updated_at!",

	c.name AS category_name,
	c.parent_id AS category_parent_id,

	d.name AS discount_name,
	d.description AS discount_description,
	d.discount_percent AS discount_percent,
	d.active  AS discount_active,
	d.created_at AS discount_created_at,
	d.updated_at AS discount_updated_at,

	pi.quantity AS "inventory_quantity?",
	pi.created_at AS "inventory_created_at?",
	pi.updated_at AS "inventory_updated_at?"
FROM product p
LEFT JOIN category c ON c.id = category_id
LEFT JOIN discount d ON d.id = discount_id
LEFT JOIN product_inventory pi ON pi.id = inventory_id;