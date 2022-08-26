import React from "react";
import { getProducts } from "../data/products";
import { v4 } from "uuid";

const Products = () => {
  let products = getProducts();

  return (
    <div>
      {products.map((product) => {
        return <h1 key={v4()}>{product.name}</h1>;
      })}
    </div>
  );
};

export default Products;
