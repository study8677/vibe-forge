import { Table, Tag, Button, Space, Tooltip, Popconfirm } from 'antd';
import {
  EyeOutlined,
  CheckOutlined,
  CloseOutlined,
  ExportOutlined,
} from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { useAppStore } from '../../store';
import { getRiskColor, getRiskLabel, getStatusLabel, getStatusColor } from '../../utils/riskRules';
import { exportRiskReport } from '../../utils/excelParser';
import RiskBadge from '../common/RiskBadge';
import type { ListingData } from '../../types';

export default function ListingTable() {
  const {
    listings,
    filterRisk,
    setSelectedListing,
    updateListingStatus,
  } = useAppStore();

  const filtered = filterRisk
    ? listings.filter((l) => l.overallRisk === filterRisk)
    : listings;

  if (listings.length === 0) return null;

  const columns: ColumnsType<ListingData> = [
    {
      title: 'SKU',
      dataIndex: 'sku',
      width: 120,
      ellipsis: true,
      sorter: (a, b) => a.sku.localeCompare(b.sku),
    },
    {
      title: '商品标题',
      dataIndex: 'title',
      ellipsis: true,
      render: (t: string) => (
        <Tooltip title={t}>
          <span>{t || <span style={{ color: '#bfbfbf' }}>—</span>}</span>
        </Tooltip>
      ),
    },
    {
      title: '品牌',
      dataIndex: 'brand',
      width: 110,
      ellipsis: true,
    },
    {
      title: '售价',
      dataIndex: 'price',
      width: 90,
      align: 'right',
      sorter: (a, b) => a.price - b.price,
      render: (p: number) => (p > 0 ? `$${p.toFixed(2)}` : '—'),
    },
    {
      title: '风险分',
      dataIndex: 'riskScore',
      width: 85,
      align: 'center',
      sorter: (a, b) => a.riskScore - b.riskScore,
      defaultSortOrder: 'descend',
      render: (s: number, row: ListingData) => (
        <span style={{ fontWeight: 700, color: getRiskColor(row.overallRisk) }}>
          {s}
        </span>
      ),
    },
    {
      title: '风险等级',
      dataIndex: 'overallRisk',
      width: 95,
      align: 'center',
      filters: [
        { text: '高风险', value: 'high' },
        { text: '中风险', value: 'medium' },
        { text: '低风险', value: 'low' },
        { text: '通过', value: 'pass' },
      ],
      onFilter: (val, row) => row.overallRisk === val,
      render: (_: unknown, row: ListingData) => <RiskBadge level={row.overallRisk} />,
    },
    {
      title: '风险项',
      dataIndex: 'risks',
      width: 160,
      render: (_: unknown, row: ListingData) => {
        const h = row.risks.filter((r) => r.level === 'high').length;
        const m = row.risks.filter((r) => r.level === 'medium').length;
        const l = row.risks.filter((r) => r.level === 'low').length;
        return (
          <Space size={4}>
            {h > 0 && <Tag color="red">{h} 高</Tag>}
            {m > 0 && <Tag color="orange">{m} 中</Tag>}
            {l > 0 && <Tag color="blue">{l} 低</Tag>}
            {h === 0 && m === 0 && l === 0 && <Tag color="green">无</Tag>}
          </Space>
        );
      },
    },
    {
      title: '状态',
      dataIndex: 'reviewStatus',
      width: 90,
      align: 'center',
      render: (s: ListingData['reviewStatus']) => (
        <Tag color={getStatusColor(s)}>{getStatusLabel(s)}</Tag>
      ),
    },
    {
      title: '操作',
      width: 160,
      align: 'center',
      fixed: 'right',
      render: (_: unknown, row: ListingData) => (
        <Space size={4}>
          <Tooltip title="查看详情">
            <Button
              type="text"
              size="small"
              icon={<EyeOutlined />}
              onClick={() => setSelectedListing(row)}
            />
          </Tooltip>
          <Tooltip title="通过">
            <Button
              type="text"
              size="small"
              icon={<CheckOutlined style={{ color: '#52c41a' }} />}
              onClick={() => updateListingStatus(row.id, 'approved')}
            />
          </Tooltip>
          <Popconfirm
            title="确认驳回此商品？"
            onConfirm={() => updateListingStatus(row.id, 'rejected')}
            okText="驳回"
            cancelText="取消"
            okButtonProps={{ danger: true }}
          >
            <Tooltip title="驳回">
              <Button
                type="text"
                size="small"
                icon={<CloseOutlined style={{ color: '#ff4d4f' }} />}
              />
            </Tooltip>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <div style={{ background: '#fff', borderRadius: 12, padding: 20 }}>
      <div className="flex items-center justify-between mb-3">
        <span style={{ fontSize: 15, fontWeight: 600 }}>
          商品列表
          {filterRisk && (
            <Tag
              color={getRiskColor(filterRisk as any)}
              closable
              onClose={() => useAppStore.getState().setFilterRisk(null)}
              style={{ marginLeft: 8 }}
            >
              {getRiskLabel(filterRisk as any)}
            </Tag>
          )}
          <span style={{ fontWeight: 400, color: '#8c8c8c', marginLeft: 8, fontSize: 13 }}>
            {filtered.length} 条
          </span>
        </span>

        <Button
          icon={<ExportOutlined />}
          onClick={() => exportRiskReport(filtered)}
        >
          导出报告
        </Button>
      </div>

      <Table<ListingData>
        rowKey="id"
        columns={columns}
        dataSource={filtered}
        size="middle"
        scroll={{ x: 1100 }}
        pagination={{
          defaultPageSize: 20,
          showSizeChanger: true,
          showTotal: (t) => `共 ${t} 条`,
        }}
        rowClassName={(row) =>
          row.overallRisk === 'high'
            ? 'row-high-risk'
            : row.reviewStatus === 'approved'
              ? 'row-approved'
              : ''
        }
        onRow={(row) => ({
          onDoubleClick: () => setSelectedListing(row),
          style: { cursor: 'pointer' },
        })}
      />
    </div>
  );
}
