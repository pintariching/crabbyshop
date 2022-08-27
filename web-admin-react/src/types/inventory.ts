export type Inventory = {
  id: number;
  quantity: number;
  created_at: Date;
  update_at: Date;
};

export type InventoryInsert = {
  quantity?: number;
};

export type InventoryUpdate = {
  quantity?: number;
};
