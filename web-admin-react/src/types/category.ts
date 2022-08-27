export type Category = {
  id: number;
  name: string;
  parent_id?: number;
  children: Category[];
};

export type CategoryInsert = {
  name?: string;
  parent_id?: number;
};

export type CategoryUpdate = {
  name?: string;
  parent_id?: number;
};
