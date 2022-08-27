export type Discount = {
  id: number;
  name: string;
  description?: string;
  discount_percent?: number;
  active: boolean;
  created_at: Date;
  updated_at: Date;
};

export type DiscountInsert = {
  name?: string;
  description?: string;
  discount_percent?: number;
  active: boolean;
};

export type DiscountUpdate = {
  name?: string;
  description?: string;
  discount_percent?: number;
  active?: boolean;
};
