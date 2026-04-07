const names = [
  'linuxer', 'neo_hacker', 'archbtw', 'void_ptr', 'sys_admin',
  'kernel_dev', 'rust_crab', 'vim_ninja', 'tmux_lord', 'nix_witch',
  'docker_whale', 'git_wizard', 'bash_guru', 'ssh_agent', 'tcp_syn',
  'zsh_power', 'emacs_user', 'pipe_dream', 'regex_god', 'chmod777',
]

const avatarColor = (name) => {
  let h = 0
  for (let i = 0; i < name.length; i++) h = name.charCodeAt(i) + ((h << 5) - h)
  return `hsl(${h % 360}, 60%, 45%)`
}

export const generateAvatar = (name) => ({
  name,
  color: avatarColor(name),
  letter: name[0].toUpperCase(),
})

const tagPool = [
  'linux', 'docker', 'rust', 'go', 'python', 'ai', 'chatgpt', 'claude',
  'gemini', 'vps', 'nas', 'homelab', 'kubernetes', 'nginx', 'wireguard',
  'tailscale', 'cloudflare', 'self-hosted', 'open-source', 'terminal',
]

const topicTemplates = [
  { title: '分享一个基于 Rust 的高性能终端文件管理器', catId: 1, tags: ['rust', 'terminal'] },
  { title: 'Docker Compose 一键部署 Immich 自建相册方案', catId: 1, tags: ['docker', 'self-hosted'] },
  { title: '2024 年 VPS 性能横评：搬瓦工 vs Racknerd vs DMIT', catId: 3, tags: ['vps'] },
  { title: 'Tailscale 组网实战：从零搭建安全内网穿透', catId: 1, tags: ['tailscale', 'wireguard'] },
  { title: 'Claude 3.5 Sonnet 深度体验：代码能力全面超越 GPT-4', catId: 8, tags: ['claude', 'ai'] },
  { title: 'NixOS 从入门到放弃再到真香的心路历程', catId: 1, tags: ['linux', 'open-source'] },
  { title: '白嫖 Cloudflare Workers 搭建反代加速节点', catId: 10, tags: ['cloudflare'] },
  { title: 'Kubernetes 集群监控实战：Prometheus + Grafana 全栈方案', catId: 1, tags: ['kubernetes'] },
  { title: '自建 NAS 避坑指南：硬件选型与系统推荐', catId: 3, tags: ['nas', 'homelab'] },
  { title: 'OpenAI o1 模型推理能力实测：数学和编程场景对比', catId: 8, tags: ['ai', 'chatgpt'] },
  { title: 'Arch Linux 滚动更新翻车记录与修复方案', catId: 1, tags: ['linux'] },
  { title: '推荐几个 GitHub 上被低估的开源项目', catId: 3, tags: ['open-source'] },
  { title: 'WireGuard 性能对比 OpenVPN：延迟和吞吐量实测', catId: 1, tags: ['wireguard'] },
  { title: 'Go 1.22 新特性解读：range over func 终于来了', catId: 1, tags: ['go'] },
  { title: '甲骨文 ARM 免费实例申请全攻略 2024 版', catId: 10, tags: ['vps'] },
  { title: 'Python 类型标注最佳实践：从 Any 到 Protocol', catId: 1, tags: ['python'] },
  { title: '论坛新功能上线：支持 Markdown 实时预览', catId: 13, tags: [] },
  { title: 'Gemini 1.5 Pro 百万 Token 上下文实际效果测评', catId: 8, tags: ['gemini', 'ai'] },
  { title: '求推荐适合做 HomeLab 的静音小主机', catId: 11, tags: ['homelab', 'nas'] },
  { title: 'Nginx 配置 HTTP/3 (QUIC) 完整教程', catId: 1, tags: ['nginx'] },
  { title: '分享我的 Neovim 配置：LazyVim + Copilot 开发体验', catId: 1, tags: ['terminal', 'open-source'] },
  { title: '国产 AI 大模型能力排行：深度对比六款主流模型', catId: 2, tags: ['ai'] },
  { title: '黑群晖 DSM 7.2 安装与硬件直通配置', catId: 3, tags: ['nas', 'self-hosted'] },
  { title: 'RSS 复兴：2024 年最佳 RSS 阅读器横评', catId: 3, tags: ['open-source'] },
  { title: '白嫖 Oracle Cloud 永久免费 VPS 的正确姿势', catId: 10, tags: ['vps'] },
  { title: 'Linux 内核 6.8 新特性一览：Intel 和 AMD 用户必看', catId: 1, tags: ['linux'] },
  { title: '用 Caddy 替代 Nginx：更简单的反向代理方案', catId: 1, tags: ['self-hosted'] },
  { title: 'AdGuard Home vs Pi-hole：DNS 去广告方案对决', catId: 3, tags: ['self-hosted', 'homelab'] },
  { title: '求助：ZFS 池损坏数据恢复的可行方案', catId: 11, tags: ['linux', 'nas'] },
  { title: '分享一年来的 HomeLab 用电成本统计', catId: 11, tags: ['homelab'] },
  { title: 'Cloudflare Tunnel 替代 frp 做内网穿透体验', catId: 1, tags: ['cloudflare', 'self-hosted'] },
  { title: '招聘：远程全栈工程师，Rust/Go 优先', catId: 6, tags: ['rust', 'go'] },
  { title: 'Proxmox VE 8.0 虚拟化平台搭建全记录', catId: 1, tags: ['homelab', 'self-hosted'] },
  { title: '关于升级论坛 Discourse 版本的通知', catId: 14, tags: [] },
  { title: '悬赏：帮忙调试一个 eBPF 程序的内存泄漏', catId: 16, tags: ['linux'] },
  { title: 'Ollama + Open WebUI 本地部署开源大模型教程', catId: 1, tags: ['ai', 'self-hosted'] },
  { title: '分享几个实用的 Shell 单行命令', catId: 1, tags: ['linux', 'terminal'] },
  { title: 'macOS Sequoia 对开发者的影响与适配指南', catId: 1, tags: ['open-source'] },
  { title: 'Hetzner 新加坡机房测评：亚太用户新选择', catId: 3, tags: ['vps'] },
  { title: '用 Zig 重写了一个 C 库，性能提升 40%', catId: 1, tags: ['open-source'] },
]

const rand = (min, max) => Math.floor(Math.random() * (max - min + 1)) + min
const pick = (arr) => arr[rand(0, arr.length - 1)]
const pickN = (arr, n) => [...arr].sort(() => Math.random() - 0.5).slice(0, n)

const timeUnits = ['分钟', '小时', '天']
const genTime = () => {
  const unit = pick(timeUnits)
  const val = unit === '分钟' ? rand(1, 59) : unit === '小时' ? rand(1, 23) : rand(1, 30)
  return `${val}${unit}前`
}

export const topics = topicTemplates.map((t, i) => ({
  id: i + 1,
  ...t,
  pinned: i < 2,
  replies: rand(0, 320),
  views: rand(50, 25000),
  activity: genTime(),
  posters: pickN(names, rand(2, 5)).map(generateAvatar),
  op: generateAvatar(pick(names)),
  isNew: Math.random() > 0.8,
  isHot: Math.random() > 0.85,
}))
