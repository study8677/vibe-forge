import { Card, Statistic } from 'antd';
import type { ReactNode } from 'react';

interface Props {
  title: string;
  value: number;
  color?: string;
  icon?: ReactNode;
  suffix?: string;
  onClick?: () => void;
  active?: boolean;
}

export default function StatCard({ title, value, color, icon, suffix, onClick, active }: Props) {
  return (
    <Card
      hoverable={!!onClick}
      onClick={onClick}
      style={{
        borderRadius: 12,
        cursor: onClick ? 'pointer' : 'default',
        borderColor: active ? color : undefined,
        borderWidth: active ? 2 : 1,
        transition: 'all 0.2s',
      }}
      styles={{ body: { padding: '16px 20px' } }}
    >
      <Statistic
        title={
          <span style={{ fontSize: 13, color: '#8c8c8c' }}>
            {icon && <span style={{ marginRight: 6 }}>{icon}</span>}
            {title}
          </span>
        }
        value={value}
        suffix={suffix}
        valueStyle={{ color: color ?? '#262626', fontSize: 28, fontWeight: 700 }}
      />
    </Card>
  );
}
