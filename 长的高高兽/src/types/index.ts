export type RiskLevel = 'high' | 'medium' | 'low' | 'pass';

export interface RiskItem {
  ruleId: string;
  ruleName: string;
  level: RiskLevel;
  message: string;
  suggestion: string;
  field: string;
}

export interface ListingData {
  id: string;
  sku: string;
  title: string;
  brand: string;
  category: string;
  price: number;
  currency: string;
  bulletPoints: string[];
  searchTerms: string;
  asin: string;
  imageUrls: string[];
  description: string;
  risks: RiskItem[];
  riskScore: number;
  overallRisk: RiskLevel;
  reviewStatus: 'pending' | 'reviewed' | 'approved' | 'rejected';
}

export interface ImageCheckResult {
  ruleId: string;
  ruleName: string;
  pass: boolean;
  score: number;
  message: string;
  suggestion: string;
}

export interface ImageAnalysis {
  file: File;
  url: string;
  width: number;
  height: number;
  fileSize: number;
  format: string;
  results: ImageCheckResult[];
  overallPass: boolean;
  overallScore: number;
  analyzedAt: Date;
}

export interface BatchUploadResult {
  total: number;
  parsed: number;
  failed: number;
  listings: ListingData[];
}

export interface DashboardStats {
  total: number;
  highRisk: number;
  mediumRisk: number;
  lowRisk: number;
  passed: number;
  reviewed: number;
}
