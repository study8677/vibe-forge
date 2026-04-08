export const defaultConfig = {
  version: 1,
  app: {
    title: 'Hiccup90',
    subtitle: 'NAS command center',
    searchEngine: 'https://www.google.com/search?q=%s',
    accent: '#77e0ff',
    density: 'comfortable',
    theme: 'graphite',
    background: 'aurora'
  },
  groups: [
    {
      id: 'group-media',
      name: 'Media',
      description: '影音与下载服务',
      collapsed: false
    },
    {
      id: 'group-storage',
      name: 'Storage',
      description: '文件、同步与备份',
      collapsed: false
    },
    {
      id: 'group-ops',
      name: 'Ops',
      description: '运维与网络服务',
      collapsed: false
    }
  ],
  items: [
    {
      id: 'item-jellyfin',
      groupId: 'group-media',
      title: 'Jellyfin',
      url: 'http://nas.local:8096',
      description: '媒体中心',
      icon: 'JF',
      tags: ['media', 'video'],
      tone: 'cyan'
    },
    {
      id: 'item-immich',
      groupId: 'group-storage',
      title: 'Immich',
      url: 'http://nas.local:2283',
      description: '照片归档',
      icon: 'IM',
      tags: ['photo', 'backup'],
      tone: 'violet'
    },
    {
      id: 'item-portainer',
      groupId: 'group-ops',
      title: 'Portainer',
      url: 'http://nas.local:9000',
      description: '容器管理',
      icon: 'PT',
      tags: ['docker', 'ops'],
      tone: 'amber'
    },
    {
      id: 'item-adguard-home',
      groupId: 'group-ops',
      title: 'AdGuard Home',
      url: 'http://nas.local:3000',
      description: '网络过滤',
      icon: 'AG',
      tags: ['dns', 'network'],
      tone: 'green'
    }
  ],
  widgets: [
    {
      id: 'widget-clock',
      type: 'clock',
      title: 'Local time'
    },
    {
      id: 'widget-stats',
      type: 'stats',
      title: 'Overview'
    },
    {
      id: 'widget-note',
      type: 'note',
      title: 'Today',
      content: '整理下载目录，检查阵列健康。'
    }
  ]
};
