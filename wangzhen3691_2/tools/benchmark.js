#!/usr/bin/env node
// Benchmark — measures ShiHuang Guard scanning throughput.
// Usage:  node tools/benchmark.js
// Env:    GUARD_API (default http://localhost:8080)
//         TOTAL     (default 2000)
//         CONC      (default 50)

const API  = process.env.GUARD_API || 'http://localhost:8080'
const TOTAL = parseInt(process.env.TOTAL || '2000', 10)
const CONC  = parseInt(process.env.CONC  || '50',   10)

const samples = [
  '这是一条正常弹幕~今天的视频太好看了',
  '加微信 xxx123 免费领取VIP会员！',
  'aaaaaaaaaaaaaaaaaaaaaa 刷屏测试',
  '点击链接 bit.ly/abc123 领取限时福利',
  '恭喜你获得iPhone大奖！请转账手续费',
  '看主页 私聊走 有惊喜哦',
  '自动回复: bot_test message',
  '我的手机号是13800138000请联系我',
  '今天天气真好，适合出门散步',
  '这个UP主做的视频质量越来越高了',
]

async function scan(text) {
  const r = await fetch(`${API}/api/scan`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ text, source: 'benchmark' }),
  })
  return r.json()
}

async function main() {
  console.log(`\n  ShiHuang Guard Benchmark`)
  console.log(`  ${API}  ·  ${TOTAL} requests  ·  ${CONC} concurrent\n`)

  let done = 0, alerts = 0, errors = 0
  const t0 = Date.now()

  while (done < TOTAL) {
    const batch = Math.min(CONC, TOTAL - done)
    const ps = Array.from({ length: batch }, () => {
      const text = samples[Math.floor(Math.random() * samples.length)]
      return scan(text)
        .then((r) => { done++; alerts += r.alerts?.length || 0 })
        .catch(() => { done++; errors++ })
    })
    await Promise.all(ps)
    process.stdout.write(`\r  progress: ${done}/${TOTAL}`)
  }

  const elapsed = (Date.now() - t0) / 1000
  console.log(`\n\n  Results`)
  console.log(`  ─────────────────────────`)
  console.log(`  requests : ${done}`)
  console.log(`  alerts   : ${alerts}`)
  console.log(`  errors   : ${errors}`)
  console.log(`  time     : ${elapsed.toFixed(2)}s`)
  console.log(`  rps      : ${(done / elapsed).toFixed(0)} req/s`)
  console.log()
}

main().catch(console.error)
