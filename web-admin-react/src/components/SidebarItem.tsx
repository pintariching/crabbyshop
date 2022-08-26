import React from "react";
import { Link } from "react-router-dom";

interface SidebarItemProps {
  to: string;
  label: string;
}

const SidebarItem = ({ to, label }: SidebarItemProps) => {
  return (
    <li>
      <Link
        to={to}
        className="flex items-center p-2 text-white hover:bg-gray-700"
      >
        <span className="flex-1 ml-3 text-left whitespace-nowrap">{label}</span>
      </Link>
    </li>
  );
};

export default SidebarItem;
