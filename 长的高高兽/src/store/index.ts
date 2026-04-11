import { create } from 'zustand';
import type { ListingData, ImageAnalysis, DashboardStats } from '../types';

interface AppState {
  listings: ListingData[];
  selectedListing: ListingData | null;
  drawerVisible: boolean;
  filterRisk: string | null;

  imageAnalyses: ImageAnalysis[];
  currentImage: ImageAnalysis | null;

  setListings: (listings: ListingData[]) => void;
  addListings: (listings: ListingData[]) => void;
  setSelectedListing: (listing: ListingData | null) => void;
  setDrawerVisible: (visible: boolean) => void;
  setFilterRisk: (risk: string | null) => void;
  updateListingStatus: (id: string, status: ListingData['reviewStatus']) => void;
  clearListings: () => void;

  addImageAnalysis: (analysis: ImageAnalysis) => void;
  setCurrentImage: (analysis: ImageAnalysis | null) => void;
  removeImageAnalysis: (index: number) => void;
  clearImages: () => void;

  getStats: () => DashboardStats;
}

export const useAppStore = create<AppState>((set, get) => ({
  listings: [],
  selectedListing: null,
  drawerVisible: false,
  filterRisk: null,
  imageAnalyses: [],
  currentImage: null,

  setListings: (listings) => set({ listings }),
  addListings: (newListings) =>
    set((s) => ({ listings: [...s.listings, ...newListings] })),
  setSelectedListing: (listing) =>
    set({ selectedListing: listing, drawerVisible: !!listing }),
  setDrawerVisible: (visible) => set({ drawerVisible: visible }),
  setFilterRisk: (risk) => set({ filterRisk: risk }),
  updateListingStatus: (id, status) =>
    set((s) => ({
      listings: s.listings.map((l) =>
        l.id === id ? { ...l, reviewStatus: status } : l
      ),
    })),
  clearListings: () => set({ listings: [], selectedListing: null }),

  addImageAnalysis: (analysis) =>
    set((s) => ({
      imageAnalyses: [analysis, ...s.imageAnalyses],
      currentImage: analysis,
    })),
  setCurrentImage: (analysis) => set({ currentImage: analysis }),
  removeImageAnalysis: (index) =>
    set((s) => {
      const next = s.imageAnalyses.filter((_, i) => i !== index);
      return {
        imageAnalyses: next,
        currentImage:
          s.currentImage === s.imageAnalyses[index]
            ? next[0] ?? null
            : s.currentImage,
      };
    }),
  clearImages: () => set({ imageAnalyses: [], currentImage: null }),

  getStats: () => {
    const listings = get().listings;
    return {
      total: listings.length,
      highRisk: listings.filter((l) => l.overallRisk === 'high').length,
      mediumRisk: listings.filter((l) => l.overallRisk === 'medium').length,
      lowRisk: listings.filter((l) => l.overallRisk === 'low').length,
      passed: listings.filter((l) => l.overallRisk === 'pass').length,
      reviewed: listings.filter((l) => l.reviewStatus !== 'pending').length,
    };
  },
}));
