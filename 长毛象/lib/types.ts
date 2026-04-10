export interface Item {
  id: string;
  name: string;
  categoryId: string;
  locationId: string;
  quantity: number;
  price?: number;
  purchaseDate?: string;
  notes?: string;
  imageData?: string;
  createdAt: string;
  updatedAt: string;
}

export interface Category {
  id: string;
  name: string;
  icon: string;
  color: string;
}

export interface Location {
  id: string;
  name: string;
  description?: string;
}

export interface StoreData {
  items: Item[];
  categories: Category[];
  locations: Location[];
}
