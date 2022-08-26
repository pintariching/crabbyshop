import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import App from "./App";
import "./index.css";
import Categories from "./routes/Categories";
import Discounts from "./routes/Discounts";
import Inventory from "./routes/Inventory";
import Products from "./routes/Products";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<App />}>
          <Route path="products" element={<Products />} />
          <Route path="categories" element={<Categories />} />
          <Route path="inventory" element={<Inventory />} />
          <Route path="discounts" element={<Discounts />} />
        </Route>
      </Routes>
    </BrowserRouter>
  </React.StrictMode>
);
