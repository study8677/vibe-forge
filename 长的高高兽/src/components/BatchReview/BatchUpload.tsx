import { useState } from 'react';
import { Upload, Button, Space, message, Spin, Alert } from 'antd';
import {
  UploadOutlined,
  DownloadOutlined,
  ExperimentOutlined,
  DeleteOutlined,
} from '@ant-design/icons';
import type { UploadFile } from 'antd';
import { useAppStore } from '../../store';
import { parseExcelFile, parseCSVFile, generateSampleTemplate } from '../../utils/excelParser';
import { analyzeListing } from '../../utils/riskRules';

export default function BatchUpload() {
  const { addListings, clearListings, listings } = useAppStore();
  const [loading, setLoading] = useState(false);

  const handleFile = async (file: File) => {
    setLoading(true);
    try {
      const ext = file.name.split('.').pop()?.toLowerCase();
      const result =
        ext === 'csv'
          ? await parseCSVFile(file)
          : await parseExcelFile(file);

      if (result.listings.length === 0) {
        message.warning('未解析到有效商品数据，请检查文件格式');
      } else {
        addListings(result.listings);
        message.success(
          `成功解析 ${result.parsed} 条商品${result.failed ? `，${result.failed} 条失败` : ''}`
        );
      }
    } catch {
      message.error('文件解析失败，请确认格式正确');
    } finally {
      setLoading(false);
    }
  };

  const loadDemo = () => {
    const demoRows: Record<string, string>[] = [
      {
        SKU: 'WB-001', Title: 'Premium Stainless Steel Water Bottle 32oz - Vacuum Insulated Wide Mouth',
        Brand: 'HydroMax', Category: 'Sports & Outdoors', Price: '24.99',
        'Bullet Point 1': 'DOUBLE-WALL VACUUM INSULATION keeps cold 24h / hot 12h',
        'Bullet Point 2': 'Premium 18/8 stainless steel, BPA-free', 'Bullet Point 3': 'Leak-proof lid with carry handle',
        'Bullet Point 4': 'Wide mouth fits ice cubes', 'Bullet Point 5': 'Perfect 32oz for gym, hiking, office',
        'Search Terms': 'water bottle insulated stainless steel gym', Description: 'Premium water bottle for active lifestyles.',
      },
      {
        SKU: 'PH-002', Title: 'BEST SELLER!!! iPhone Case ★★★ FREE SHIPPING Nike Style 最好',
        Brand: 'nike', Category: 'Electronics', Price: '0.50',
        'Bullet Point 1': '<b>Amazing quality!</b>', 'Bullet Point 2': '', 'Bullet Point 3': '',
        'Bullet Point 4': '', 'Bullet Point 5': '',
        'Search Terms': 'iphone case apple samsung B0ABC12345',
        Description: 'Best iPhone case! Email seller@shop.com or call 123-456-7890 for wholesale.',
      },
      {
        SKU: 'CB-003', Title: 'Bamboo Cutting Board Set 3-Piece Kitchen Chopping Boards with Juice Groove',
        Brand: 'EcoChef', Category: 'Kitchen & Dining', Price: '29.99',
        'Bullet Point 1': '100% organic bamboo, sustainable', 'Bullet Point 2': 'Set of 3 sizes',
        'Bullet Point 3': 'Deep juice grooves', 'Bullet Point 4': '', 'Bullet Point 5': '',
        'Search Terms': 'cutting board bamboo kitchen set organic',
        Description: 'EcoChef bamboo cutting board set for your kitchen.',
      },
      {
        SKU: 'LP-004', Title: 'LED Desk Lamp with USB Charging Port, 5 Brightness Levels, Touch Control',
        Brand: 'LumiDesk', Category: 'Office Products', Price: '35.99',
        'Bullet Point 1': '5 brightness levels + 3 color temperatures',
        'Bullet Point 2': 'Built-in USB charging port', 'Bullet Point 3': 'Touch-sensitive control panel',
        'Bullet Point 4': 'Flexible gooseneck arm', 'Bullet Point 5': 'Energy-efficient LED, 50000h lifespan',
        'Search Terms': 'desk lamp LED USB charging adjustable office study',
        Description: 'LumiDesk LED lamp designed for comfortable reading and working.',
      },
      {
        SKU: '', Title: 'Yoga Mat',
        Brand: '', Category: 'Sports Collectibles', Price: '2.00',
        'Bullet Point 1': '', 'Bullet Point 2': '', 'Bullet Point 3': '',
        'Bullet Point 4': '', 'Bullet Point 5': '',
        'Search Terms': '', Description: '',
      },
      {
        SKU: 'BG-006', Title: 'Canvas Tote Bag Large Capacity Reusable Shopping Bag Eco Friendly',
        Brand: 'GreenCarry', Category: 'Clothing, Shoes & Jewelry', Price: '15.99',
        'Bullet Point 1': 'Extra-large capacity fits weekly groceries',
        'Bullet Point 2': 'Heavy-duty 12oz canvas, machine washable',
        'Bullet Point 3': 'Reinforced stitching on handles',
        'Bullet Point 4': 'Interior zipper pocket for keys and phone',
        'Bullet Point 5': 'Folds flat for easy storage',
        'Search Terms': 'tote bag canvas reusable shopping eco friendly large',
        Description: 'GreenCarry canvas tote bag — stylish, durable, and eco-conscious.',
      },
    ];

    const listings = demoRows.map((r) => analyzeListing(r));
    addListings(listings);
    message.success(`已加载 ${listings.length} 条演示数据`);
  };

  return (
    <div
      style={{
        background: '#fff',
        borderRadius: 12,
        padding: 24,
        marginBottom: 20,
      }}
    >
      <Spin spinning={loading} tip="正在解析文件...">
        <div className="flex items-center justify-between flex-wrap gap-3">
          <Space size="middle" wrap>
            <Upload
              accept=".xlsx,.xls,.csv"
              showUploadList={false}
              beforeUpload={(file: UploadFile) => {
                handleFile(file as unknown as File);
                return false;
              }}
            >
              <Button type="primary" icon={<UploadOutlined />} size="large">
                上传商品文件
              </Button>
            </Upload>

            <Button icon={<ExperimentOutlined />} onClick={loadDemo} size="large">
              加载演示数据
            </Button>

            <Button icon={<DownloadOutlined />} onClick={generateSampleTemplate} size="large">
              下载模板
            </Button>

            {listings.length > 0 && (
              <Button
                danger
                icon={<DeleteOutlined />}
                onClick={() => {
                  clearListings();
                  message.info('已清空');
                }}
                size="large"
              >
                清空数据
              </Button>
            )}
          </Space>

          <span style={{ color: '#8c8c8c', fontSize: 13 }}>
            支持 .xlsx / .xls / .csv，列名兼容中英文
          </span>
        </div>
      </Spin>

      {listings.length === 0 && (
        <Alert
          type="info"
          showIcon
          message="快速开始"
          description="上传包含 SKU、Title、Brand、Category、Price 等列的表格文件，或点击「加载演示数据」立即体验风险筛查。"
          style={{ marginTop: 16, borderRadius: 8 }}
        />
      )}
    </div>
  );
}
