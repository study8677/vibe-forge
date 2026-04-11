import { Drawer, Descriptions, List, Tag, Button, Space, Divider, Progress, Empty } from 'antd';
import {
  CheckOutlined,
  CloseOutlined,
  AlertOutlined,
  WarningOutlined,
  InfoCircleOutlined,
} from '@ant-design/icons';
import { useAppStore } from '../../store';
import { getRiskColor, getRiskLabel } from '../../utils/riskRules';
import RiskBadge from '../common/RiskBadge';
import type { RiskItem } from '../../types';

const LEVEL_ICON = {
  high: <AlertOutlined style={{ color: '#ff4d4f' }} />,
  medium: <WarningOutlined style={{ color: '#faad14' }} />,
  low: <InfoCircleOutlined style={{ color: '#1677ff' }} />,
  pass: <CheckOutlined style={{ color: '#52c41a' }} />,
};

export default function RiskDetailDrawer() {
  const { selectedListing: listing, drawerVisible, setDrawerVisible, updateListingStatus } =
    useAppStore();

  if (!listing) return null;

  const grouped: Record<string, RiskItem[]> = {};
  for (const r of listing.risks) {
    (grouped[r.field] ??= []).push(r);
  }

  const fieldLabel: Record<string, string> = {
    title: '标题',
    brand: '品牌',
    price: '价格',
    category: '类目',
    bulletPoints: '五点描述',
    searchTerms: '搜索词',
    description: '商品描述',
    sku: 'SKU',
  };

  return (
    <Drawer
      title={
        <div className="flex items-center gap-2">
          <span>风险详情</span>
          <RiskBadge level={listing.overallRisk} />
        </div>
      }
      width={560}
      open={drawerVisible}
      onClose={() => setDrawerVisible(false)}
      extra={
        <Space>
          <Button
            type="primary"
            icon={<CheckOutlined />}
            onClick={() => {
              updateListingStatus(listing.id, 'approved');
              setDrawerVisible(false);
            }}
          >
            通过
          </Button>
          <Button
            danger
            icon={<CloseOutlined />}
            onClick={() => {
              updateListingStatus(listing.id, 'rejected');
              setDrawerVisible(false);
            }}
          >
            驳回
          </Button>
        </Space>
      }
    >
      {/* 基本信息 */}
      <Descriptions
        column={2}
        size="small"
        bordered
        style={{ marginBottom: 20 }}
        styles={{ label: { fontWeight: 600, width: 80 } }}
      >
        <Descriptions.Item label="SKU">{listing.sku || '—'}</Descriptions.Item>
        <Descriptions.Item label="品牌">{listing.brand || '—'}</Descriptions.Item>
        <Descriptions.Item label="售价" span={1}>
          {listing.price > 0 ? `$${listing.price.toFixed(2)}` : '—'}
        </Descriptions.Item>
        <Descriptions.Item label="类目">{listing.category || '—'}</Descriptions.Item>
        <Descriptions.Item label="标题" span={2}>
          <span style={{ wordBreak: 'break-all' }}>{listing.title || '—'}</span>
        </Descriptions.Item>
      </Descriptions>

      {/* 风险分数 */}
      <div className="flex items-center gap-4 mb-4">
        <Progress
          type="dashboard"
          percent={listing.riskScore}
          size={80}
          strokeColor={getRiskColor(listing.overallRisk)}
          format={(pct) => (
            <span style={{ fontSize: 18, fontWeight: 700, color: getRiskColor(listing.overallRisk) }}>
              {pct}
            </span>
          )}
        />
        <div>
          <div style={{ fontSize: 15, fontWeight: 600 }}>
            风险评分 {listing.riskScore} / 100
          </div>
          <div style={{ color: '#8c8c8c', fontSize: 13 }}>
            {listing.risks.length} 项风险，
            {listing.risks.filter((r) => r.level === 'high').length} 高 /
            {listing.risks.filter((r) => r.level === 'medium').length} 中 /
            {listing.risks.filter((r) => r.level === 'low').length} 低
          </div>
        </div>
      </div>

      <Divider style={{ margin: '16px 0' }} />

      {/* 按字段分组的风险列表 */}
      {listing.risks.length === 0 ? (
        <Empty description="所有检测项已通过" image={Empty.PRESENTED_IMAGE_SIMPLE} />
      ) : (
        Object.entries(grouped).map(([field, items]) => (
          <div key={field} style={{ marginBottom: 16 }}>
            <div
              style={{
                fontSize: 13,
                fontWeight: 600,
                color: '#595959',
                marginBottom: 8,
                padding: '4px 8px',
                background: '#fafafa',
                borderRadius: 6,
              }}
            >
              {fieldLabel[field] ?? field}
            </div>
            <List
              size="small"
              dataSource={items}
              renderItem={(item) => (
                <List.Item style={{ padding: '8px 0', borderBottom: '1px solid #f5f5f5' }}>
                  <div style={{ width: '100%' }}>
                    <div className="flex items-start gap-2">
                      {LEVEL_ICON[item.level]}
                      <div style={{ flex: 1 }}>
                        <div className="flex items-center gap-2">
                          <span style={{ fontWeight: 600, fontSize: 13 }}>{item.ruleName}</span>
                          <Tag
                            color={getRiskColor(item.level)}
                            style={{ fontSize: 11, lineHeight: '18px', padding: '0 4px' }}
                          >
                            {getRiskLabel(item.level)}
                          </Tag>
                        </div>
                        <div style={{ color: '#595959', fontSize: 13, margin: '2px 0' }}>
                          {item.message}
                        </div>
                        {item.suggestion && (
                          <div style={{ color: '#1677ff', fontSize: 12 }}>
                            建议：{item.suggestion}
                          </div>
                        )}
                      </div>
                    </div>
                  </div>
                </List.Item>
              )}
            />
          </div>
        ))
      )}

      {/* 五点描述 */}
      {listing.bulletPoints.length > 0 && (
        <>
          <Divider style={{ margin: '16px 0' }} />
          <div style={{ fontSize: 13, fontWeight: 600, marginBottom: 8 }}>Bullet Points</div>
          <ol style={{ paddingLeft: 20, margin: 0, color: '#595959', fontSize: 13 }}>
            {listing.bulletPoints.map((bp, i) => (
              <li key={i} style={{ marginBottom: 4 }}>{bp}</li>
            ))}
          </ol>
        </>
      )}

      {/* 搜索词 */}
      {listing.searchTerms && (
        <>
          <Divider style={{ margin: '16px 0' }} />
          <div style={{ fontSize: 13, fontWeight: 600, marginBottom: 8 }}>Search Terms</div>
          <div style={{ color: '#595959', fontSize: 13, wordBreak: 'break-all' }}>
            {listing.searchTerms}
          </div>
        </>
      )}
    </Drawer>
  );
}
