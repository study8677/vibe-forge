import * as XLSX from 'xlsx';
import type { ListingData, BatchUploadResult } from '../types';
import { analyzeListing } from './riskRules';

// ── 解析 ──────────────────────────────────────────────────

export async function parseExcelFile(file: File): Promise<BatchUploadResult> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = (e) => {
      try {
        const data = new Uint8Array(e.target!.result as ArrayBuffer);
        const wb = XLSX.read(data, { type: 'array' });
        const sheet = wb.Sheets[wb.SheetNames[0]];
        const rows = XLSX.utils.sheet_to_json<Record<string, string>>(sheet, {
          raw: false,
        });

        let failed = 0;
        const listings: ListingData[] = [];
        for (const row of rows) {
          try {
            listings.push(analyzeListing(row));
          } catch {
            failed++;
          }
        }

        resolve({ total: rows.length, parsed: listings.length, failed, listings });
      } catch (err) {
        reject(err);
      }
    };
    reader.onerror = reject;
    reader.readAsArrayBuffer(file);
  });
}

export function parseCSVFile(file: File): Promise<BatchUploadResult> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = (e) => {
      try {
        const text = e.target!.result as string;
        const lines = text.split('\n').filter((l) => l.trim());
        if (lines.length < 2) {
          resolve({ total: 0, parsed: 0, failed: 0, listings: [] });
          return;
        }

        const headers = lines[0]
          .split(',')
          .map((h) => h.trim().replace(/^"|"$/g, ''));

        let failed = 0;
        const listings: ListingData[] = [];

        for (let i = 1; i < lines.length; i++) {
          try {
            const vals: string[] = [];
            let cur = '';
            let inQ = false;
            for (const ch of lines[i]) {
              if (ch === '"') inQ = !inQ;
              else if (ch === ',' && !inQ) {
                vals.push(cur.trim());
                cur = '';
              } else cur += ch;
            }
            vals.push(cur.trim());

            const row: Record<string, string> = {};
            headers.forEach((h, idx) => (row[h] = vals[idx] ?? ''));
            listings.push(analyzeListing(row));
          } catch {
            failed++;
          }
        }

        resolve({ total: lines.length - 1, parsed: listings.length, failed, listings });
      } catch (err) {
        reject(err);
      }
    };
    reader.onerror = reject;
    reader.readAsText(file);
  });
}

// ── 导出 ──────────────────────────────────────────────────

export function exportRiskReport(listings: ListingData[]): void {
  const rows = listings.map((l) => ({
    SKU: l.sku,
    商品标题: l.title,
    品牌: l.brand,
    类目: l.category,
    售价: l.price,
    风险等级: { high: '高风险', medium: '中风险', low: '低风险', pass: '通过' }[l.overallRisk],
    风险分数: l.riskScore,
    审核状态: { pending: '待审核', reviewed: '已审核', approved: '已通过', rejected: '已驳回' }[l.reviewStatus],
    风险项数: l.risks.length,
    高风险项: l.risks.filter((r) => r.level === 'high').map((r) => r.message).join('; '),
    中风险项: l.risks.filter((r) => r.level === 'medium').map((r) => r.message).join('; '),
    低风险项: l.risks.filter((r) => r.level === 'low').map((r) => r.message).join('; '),
  }));

  const ws = XLSX.utils.json_to_sheet(rows);
  ws['!cols'] = Object.keys(rows[0] ?? {}).map((k) => ({
    wch: Math.max(k.length * 2, 15),
  }));

  const wb = XLSX.utils.book_new();
  XLSX.utils.book_append_sheet(wb, ws, '风险审查报告');
  XLSX.writeFile(wb, `亚马逊上架风险报告_${new Date().toISOString().slice(0, 10)}.xlsx`);
}

// ── 模板 ──────────────────────────────────────────────────

export function generateSampleTemplate(): void {
  const sample = [
    {
      SKU: 'SAMPLE-001',
      Title: 'Premium Stainless Steel Water Bottle 32oz - Vacuum Insulated, BPA-Free, Wide Mouth',
      Brand: 'HydroMax',
      Category: 'Sports & Outdoors',
      Price: '24.99',
      'Bullet Point 1': 'DOUBLE-WALL VACUUM INSULATION keeps drinks cold 24h or hot 12h',
      'Bullet Point 2': 'PREMIUM 18/8 STAINLESS STEEL, BPA-free and toxin-free',
      'Bullet Point 3': 'LEAK-PROOF LID with easy-carry handle',
      'Bullet Point 4': 'WIDE MOUTH fits ice cubes, easy to clean',
      'Bullet Point 5': 'PERFECT 32oz for gym, hiking, office, everyday use',
      'Search Terms': 'water bottle insulated stainless steel gym hiking BPA free leak proof',
      Description: 'HydroMax premium water bottle designed for active lifestyles. Keeps your beverages at the perfect temperature all day long.',
      ASIN: '',
    },
    {
      SKU: 'SAMPLE-002',
      Title: 'BEST SELLER!!! iPhone Case ★★★ FREE SHIPPING Nike Style',
      Brand: 'nike',
      Category: 'Electronics',
      Price: '0.50',
      'Bullet Point 1': '<b>Amazing quality!</b>',
      'Bullet Point 2': '',
      'Bullet Point 3': '',
      'Bullet Point 4': '',
      'Bullet Point 5': '',
      'Search Terms': 'iphone case apple samsung B0EXAMPLE123',
      Description: 'Contact us at seller@example.com or call 123-456-7890 for bulk orders!',
      ASIN: '',
    },
    {
      SKU: 'SAMPLE-003',
      Title: 'Bamboo Cutting Board Set, 3-Piece Kitchen Chopping Boards with Juice Groove',
      Brand: 'EcoChef',
      Category: 'Kitchen & Dining',
      Price: '29.99',
      'Bullet Point 1': '100% ORGANIC BAMBOO construction, sustainable and eco-friendly',
      'Bullet Point 2': 'SET OF 3 SIZES for all kitchen prep needs',
      'Bullet Point 3': 'DEEP JUICE GROOVES prevent mess on countertops',
      'Bullet Point 4': '',
      'Bullet Point 5': '',
      'Search Terms': 'cutting board bamboo kitchen chopping board set organic eco friendly',
      Description: 'EcoChef bamboo cutting board set brings natural elegance to your kitchen.',
      ASIN: '',
    },
  ];

  const ws = XLSX.utils.json_to_sheet(sample);
  ws['!cols'] = [
    { wch: 15 }, { wch: 70 }, { wch: 15 }, { wch: 22 }, { wch: 10 },
    { wch: 55 }, { wch: 55 }, { wch: 55 }, { wch: 55 }, { wch: 55 },
    { wch: 60 }, { wch: 70 }, { wch: 15 },
  ];

  const wb = XLSX.utils.book_new();
  XLSX.utils.book_append_sheet(wb, ws, '商品数据');
  XLSX.writeFile(wb, '亚马逊上架模板.xlsx');
}
