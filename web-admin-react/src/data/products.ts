let products: { name: string; sku: string }[] = [
  {
    name: "Product 1",
    sku: "12345",
  },
  {
    name: "Product 2",
    sku: "67890",
  },
];

export const getProducts = () => {
  return products;
};
