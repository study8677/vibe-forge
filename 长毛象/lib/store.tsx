"use client";

import {
  createContext,
  useContext,
  useReducer,
  useEffect,
  useState,
  type ReactNode,
} from "react";
import type { Item, Category, Location, StoreData } from "./types";

const STORAGE_KEY = "my-items-store";

const defaultCategories: Category[] = [
  { id: "cat-1", name: "电子产品", icon: "📱", color: "#3B82F6" },
  { id: "cat-2", name: "衣物", icon: "👔", color: "#8B5CF6" },
  { id: "cat-3", name: "书籍", icon: "📚", color: "#F59E0B" },
  { id: "cat-4", name: "食品", icon: "🍎", color: "#10B981" },
  { id: "cat-5", name: "家具", icon: "🪑", color: "#6366F1" },
  { id: "cat-6", name: "厨具", icon: "🍳", color: "#EC4899" },
  { id: "cat-7", name: "工具", icon: "🔧", color: "#F97316" },
  { id: "cat-8", name: "文具", icon: "✏️", color: "#14B8A6" },
  { id: "cat-9", name: "其他", icon: "📦", color: "#6B7280" },
];

const defaultLocations: Location[] = [
  { id: "loc-1", name: "客厅" },
  { id: "loc-2", name: "卧室" },
  { id: "loc-3", name: "厨房" },
  { id: "loc-4", name: "卫生间" },
  { id: "loc-5", name: "书房" },
  { id: "loc-6", name: "阳台" },
  { id: "loc-7", name: "储物间" },
];

const defaultData: StoreData = {
  items: [],
  categories: defaultCategories,
  locations: defaultLocations,
};

type Action =
  | { type: "LOAD"; payload: StoreData }
  | { type: "ADD_ITEM"; payload: Item }
  | { type: "UPDATE_ITEM"; payload: Item }
  | { type: "DELETE_ITEM"; payload: string }
  | { type: "ADD_CATEGORY"; payload: Category }
  | { type: "UPDATE_CATEGORY"; payload: Category }
  | { type: "DELETE_CATEGORY"; payload: string }
  | { type: "ADD_LOCATION"; payload: Location }
  | { type: "UPDATE_LOCATION"; payload: Location }
  | { type: "DELETE_LOCATION"; payload: string };

function reducer(state: StoreData, action: Action): StoreData {
  switch (action.type) {
    case "LOAD":
      return action.payload;
    case "ADD_ITEM":
      return { ...state, items: [action.payload, ...state.items] };
    case "UPDATE_ITEM":
      return {
        ...state,
        items: state.items.map((i) =>
          i.id === action.payload.id ? action.payload : i
        ),
      };
    case "DELETE_ITEM":
      return {
        ...state,
        items: state.items.filter((i) => i.id !== action.payload),
      };
    case "ADD_CATEGORY":
      return {
        ...state,
        categories: [...state.categories, action.payload],
      };
    case "UPDATE_CATEGORY":
      return {
        ...state,
        categories: state.categories.map((c) =>
          c.id === action.payload.id ? action.payload : c
        ),
      };
    case "DELETE_CATEGORY":
      return {
        ...state,
        categories: state.categories.filter((c) => c.id !== action.payload),
      };
    case "ADD_LOCATION":
      return {
        ...state,
        locations: [...state.locations, action.payload],
      };
    case "UPDATE_LOCATION":
      return {
        ...state,
        locations: state.locations.map((l) =>
          l.id === action.payload.id ? action.payload : l
        ),
      };
    case "DELETE_LOCATION":
      return {
        ...state,
        locations: state.locations.filter((l) => l.id !== action.payload),
      };
    default:
      return state;
  }
}

interface StoreContextType {
  data: StoreData;
  dispatch: React.Dispatch<Action>;
  ready: boolean;
}

const StoreContext = createContext<StoreContextType>({
  data: defaultData,
  dispatch: () => {},
  ready: false,
});

export function StoreProvider({ children }: { children: ReactNode }) {
  const [data, dispatch] = useReducer(reducer, defaultData);
  const [ready, setReady] = useState(false);

  // Load from localStorage on mount
  useEffect(() => {
    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      if (stored) {
        const parsed = JSON.parse(stored) as StoreData;
        dispatch({ type: "LOAD", payload: parsed });
      }
    } catch {
      // ignore parse errors, use defaults
    }
    setReady(true);
  }, []);

  // Persist to localStorage on every change (after initial load)
  useEffect(() => {
    if (ready) {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(data));
    }
  }, [data, ready]);

  return (
    <StoreContext.Provider value={{ data, dispatch, ready }}>
      {children}
    </StoreContext.Provider>
  );
}

export function useStore() {
  return useContext(StoreContext);
}
