import React from "react";
import { Link } from "react-router-dom";
import SidebarItem from "./SidebarItem";
import { v4 } from "uuid";

const Sidebar = () => {
  const routes: { to: string; label: string }[] = [
    { to: "/", label: "Dashboard" },
    { to: "/products", label: "Products" },
    { to: "/categories", label: "Categories" },
    { to: "/inventory", label: "Inventory" },
    { to: "/discounts", label: "Discount" },
  ];
  return (
    <aside className="w-64 h-full" aria-label="sidebar">
      <div className="bg-gray-600 h-full">
        <ul className="space-y-2">
          {routes.map((route) => {
            return <SidebarItem key={v4()} to={route.to} label={route.label} />;
          })}
        </ul>
      </div>
    </aside>
  );
};

export default Sidebar;
