import { Tag } from 'antd';
import type { RiskLevel } from '../../types';
import { getRiskColor, getRiskLabel } from '../../utils/riskRules';

export default function RiskBadge({ level }: { level: RiskLevel }) {
  return (
    <Tag
      color={getRiskColor(level)}
      style={{ fontWeight: 600, borderRadius: 4, fontSize: 12 }}
    >
      {getRiskLabel(level)}
    </Tag>
  );
}
