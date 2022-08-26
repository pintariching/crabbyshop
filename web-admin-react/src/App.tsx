import React from "react";
import Navbar from "./components/Navbar";
import Sidebar from "./components/Sidebar";
import { Outlet } from "react-router-dom";

const App = () => {
  return (
    <>
      <div className="flex flex-col w-screen h-screen">
        <Navbar />
        <div className="flex flex-row w-full h-full">
          <Sidebar />
          <div className="p-16">
            <Outlet />
          </div>
        </div>
      </div>
    </>
  );
};

export default App;
