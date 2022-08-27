import { Product } from "../types/product";

let products: Product[] = [
  {
    id: 1,
    name: "Product 1",
    sku: "12345",
    price: 143,
    created_at: new Date(Date.now()),
    updated_at: new Date(Date.now()),
  },
  {
    id: 2,
    name: "Product 2",
    sku: "12345",
    price: 12.43,
    created_at: new Date(Date.now()),
    updated_at: new Date(Date.now()),
  },
  {
    id: 3,
    name: "Product 3",
    sku: "12345",
    price: 63.2,
    created_at: new Date(Date.now()),
    updated_at: new Date(Date.now()),
  },
  {
    id: 4,
    name: "Product 4",
    sku: "12345",
    price: 1.99,
    created_at: new Date(Date.now()),
    updated_at: new Date(Date.now()),
  },
  {
    id: 5,
    name: "Product 5",
    sku: "12345",
    price: 4.52,
    created_at: new Date(Date.now()),
    updated_at: new Date(Date.now()),
  },
];

export const getProducts = () => {
  return products;
};
