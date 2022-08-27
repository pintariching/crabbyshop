import { Category } from "./category";
import { Discount } from "./discount";
import { Inventory } from "./inventory";

export type Product = {
  id: number;
  name: string;
  description?: string;
  sku?: string;
  category_id?: number;
  category?: Category;
  inventory_id?: number;
  inventory?: Inventory;
  price?: number;
  discount_id?: number;
  discount?: Discount;
  created_at: Date;
  updated_at: Date;
};

export type ProductInsert = {
  name?: string;
  description?: string;
  sku?: string;
  category_id?: number;
  inventory_id?: number;
  price?: number;
  discount_id?: number;
};

export type ProductUpdate = {
  name?: string;
  description?: string;
  sku?: string;
  category_id?: number;
  inventory_id?: number;
  price?: number;
  discount_id?: number;
};
