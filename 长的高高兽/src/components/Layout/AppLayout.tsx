import { useState } from 'react';
import { Layout, Menu } from 'antd';
import {
  AuditOutlined,
  PictureOutlined,
  MenuFoldOutlined,
  MenuUnfoldOutlined,
} from '@ant-design/icons';
import { useNavigate, useLocation, Outlet } from 'react-router-dom';

const { Sider, Header, Content } = Layout;

const NAV_ITEMS = [
  { key: '/batch', icon: <AuditOutlined />, label: '批量风险筛查' },
  { key: '/image', icon: <PictureOutlined />, label: '单图快速核查' },
];

export default function AppLayout() {
  const [collapsed, setCollapsed] = useState(false);
  const navigate = useNavigate();
  const { pathname } = useLocation();

  const activeKey = NAV_ITEMS.find((n) => pathname.startsWith(n.key))?.key ?? '/batch';

  return (
    <Layout style={{ minHeight: '100vh' }}>
      <Sider
        collapsible
        collapsed={collapsed}
        onCollapse={setCollapsed}
        trigger={null}
        width={220}
        style={{
          background: '#001529',
          boxShadow: '2px 0 8px rgba(0,0,0,0.08)',
        }}
      >
        {/* Logo */}
        <div
          style={{
            height: 64,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            borderBottom: '1px solid rgba(255,255,255,0.08)',
          }}
        >
          {collapsed ? (
            <AuditOutlined style={{ fontSize: 24, color: '#fff' }} />
          ) : (
            <span style={{ color: '#fff', fontSize: 16, fontWeight: 700, letterSpacing: 1 }}>
              AMZ 审查工作台
            </span>
          )}
        </div>

        <Menu
          theme="dark"
          mode="inline"
          selectedKeys={[activeKey]}
          items={NAV_ITEMS}
          onClick={({ key }) => navigate(key)}
          style={{ borderRight: 0, marginTop: 8 }}
        />
      </Sider>

      <Layout>
        <Header
          style={{
            background: '#fff',
            padding: '0 24px',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            boxShadow: '0 1px 4px rgba(0,0,0,0.04)',
            position: 'sticky',
            top: 0,
            zIndex: 100,
            height: 56,
          }}
        >
          <div className="flex items-center gap-3">
            <span
              onClick={() => setCollapsed(!collapsed)}
              style={{ fontSize: 18, cursor: 'pointer', color: '#595959' }}
            >
              {collapsed ? <MenuUnfoldOutlined /> : <MenuFoldOutlined />}
            </span>
            <span style={{ fontSize: 15, fontWeight: 600, color: '#262626' }}>
              {NAV_ITEMS.find((n) => n.key === activeKey)?.label}
            </span>
          </div>

          <span style={{ fontSize: 12, color: '#bfbfbf' }}>
            Amazon Cross-Border Listing Review Workbench
          </span>
        </Header>

        <Content
          style={{
            padding: 20,
            background: '#f5f5f5',
            overflow: 'auto',
          }}
        >
          <Outlet />
        </Content>
      </Layout>
    </Layout>
  );
}
