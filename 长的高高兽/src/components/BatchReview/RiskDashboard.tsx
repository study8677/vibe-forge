import { Row, Col } from 'antd';
import {
  AlertOutlined,
  WarningOutlined,
  InfoCircleOutlined,
  CheckCircleOutlined,
  AppstoreOutlined,
} from '@ant-design/icons';
import { PieChart, Pie, Cell, Tooltip, ResponsiveContainer } from 'recharts';
import { useAppStore } from '../../store';
import StatCard from '../common/StatCard';

const COLORS = {
  high: '#ff4d4f',
  medium: '#faad14',
  low: '#1677ff',
  pass: '#52c41a',
};

export default function RiskDashboard() {
  const { listings, filterRisk, setFilterRisk } = useAppStore();
  const stats = useAppStore((s) => s.getStats());

  if (listings.length === 0) return null;

  const pieData = [
    { name: '高风险', value: stats.highRisk, color: COLORS.high },
    { name: '中风险', value: stats.mediumRisk, color: COLORS.medium },
    { name: '低风险', value: stats.lowRisk, color: COLORS.low },
    { name: '通过', value: stats.passed, color: COLORS.pass },
  ].filter((d) => d.value > 0);

  const toggle = (key: string | null) =>
    setFilterRisk(filterRisk === key ? null : key);

  return (
    <div style={{ marginBottom: 20 }}>
      <Row gutter={[16, 16]} align="middle">
        <Col xs={24} sm={12} md={4}>
          <StatCard
            title="总计"
            value={stats.total}
            icon={<AppstoreOutlined />}
            onClick={() => toggle(null)}
            active={filterRisk === null}
          />
        </Col>
        <Col xs={12} sm={12} md={4}>
          <StatCard
            title="高风险"
            value={stats.highRisk}
            color={COLORS.high}
            icon={<AlertOutlined />}
            onClick={() => toggle('high')}
            active={filterRisk === 'high'}
          />
        </Col>
        <Col xs={12} sm={12} md={4}>
          <StatCard
            title="中风险"
            value={stats.mediumRisk}
            color={COLORS.medium}
            icon={<WarningOutlined />}
            onClick={() => toggle('medium')}
            active={filterRisk === 'medium'}
          />
        </Col>
        <Col xs={12} sm={12} md={4}>
          <StatCard
            title="低风险"
            value={stats.lowRisk}
            color={COLORS.low}
            icon={<InfoCircleOutlined />}
            onClick={() => toggle('low')}
            active={filterRisk === 'low'}
          />
        </Col>
        <Col xs={12} sm={12} md={4}>
          <StatCard
            title="通过"
            value={stats.passed}
            color={COLORS.pass}
            icon={<CheckCircleOutlined />}
            onClick={() => toggle('pass')}
            active={filterRisk === 'pass'}
          />
        </Col>
        <Col xs={24} md={4}>
          <div
            style={{
              background: '#fff',
              borderRadius: 12,
              padding: '8px 0',
              display: 'flex',
              justifyContent: 'center',
            }}
          >
            <ResponsiveContainer width={120} height={100}>
              <PieChart>
                <Pie
                  data={pieData}
                  cx="50%"
                  cy="50%"
                  innerRadius={28}
                  outerRadius={44}
                  dataKey="value"
                  stroke="none"
                >
                  {pieData.map((d, i) => (
                    <Cell key={i} fill={d.color} />
                  ))}
                </Pie>
                <Tooltip
                  formatter={(value: number, name: string) => [`${value} 条`, name]}
                />
              </PieChart>
            </ResponsiveContainer>
          </div>
        </Col>
      </Row>
    </div>
  );
}
