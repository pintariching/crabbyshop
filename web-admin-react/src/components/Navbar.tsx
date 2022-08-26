import React from "react";
import { Link } from "react-router-dom";

const Navbar = () => {
  return (
    <nav className="bg-blue-700 py-4" aria-label="navbar">
      <div className="flex justify-between mx-16">
        <Link to="/" className="flex items-center">
          <span className="self-center text-2xl font-semibold text-white">
            Crabbyshop
          </span>
        </Link>
        <Link to="/login" className="flex items-center">
          <span className="self-center text-xl text-white">Login</span>
        </Link>
      </div>
    </nav>
  );
};

export default Navbar;
