import React, { useState } from "react";
import {
  createColumnHelper,
  flexRender,
  getCoreRowModel,
  useReactTable,
} from "@tanstack/react-table";
import { getProducts } from "../data/products";
import { v4 } from "uuid";
import { Product } from "../types/product";

const columnHelper = createColumnHelper<Product>();

const columns = [
  columnHelper.accessor("id", {
    header: () => <span>Id</span>,
    cell: (info) => info.getValue(),
  }),
  columnHelper.accessor("name", {
    header: () => <span>Name</span>,
    cell: (info) => info.getValue(),
  }),
  columnHelper.accessor("sku", {
    header: () => <span>SKU</span>,
    cell: (info) => info.getValue(),
  }),
  columnHelper.accessor("price", {
    header: () => <span>Price</span>,
    cell: (info) => info.getValue(),
  }),
];

const Products = () => {
  const [data, setData] = useState(() => [...getProducts()]);

  const table = useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
  });

  return (
    <div className="p-2">
      <table className="border-2 border-black">
        <thead>
          {table.getHeaderGroups().map((headerGroup) => (
            <tr key={headerGroup.id}>
              {headerGroup.headers.map((header) => (
                <th
                  key={header.id}
                  className="border-b-2 border-r-2 border-black py-2 px-4"
                >
                  {header.isPlaceholder
                    ? null
                    : flexRender(
                        header.column.columnDef.header,
                        header.getContext()
                      )}
                </th>
              ))}
            </tr>
          ))}
        </thead>
        <tbody>
          {table.getRowModel().rows.map((row) => (
            <tr key={row.id}>
              {row.getVisibleCells().map((cell) => (
                <td
                  key={cell.id}
                  className="border-b border-r-2 border-black py-2 px-4"
                >
                  {flexRender(cell.column.columnDef.cell, cell.getContext())}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default Products;
