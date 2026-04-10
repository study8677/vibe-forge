"""浏览器管理 — Playwright 封装，含 Cookie 持久化与反检测"""

from __future__ import annotations

import asyncio
import json
import random
from pathlib import Path
from typing import Optional

from playwright.async_api import (
    async_playwright,
    Browser,
    BrowserContext,
    Page,
    Playwright,
)

import config as cfg


class BrowserManager:
    """管理 Playwright 浏览器生命周期"""

    def __init__(self):
        self._pw: Optional[Playwright] = None
        self._browser: Optional[Browser] = None
        self._contexts: dict[str, BrowserContext] = {}

    # ── 生命周期 ──────────────────────────────────────

    async def start(self):
        self._pw = await async_playwright().start()
        self._browser = await self._pw.chromium.launch(
            headless=cfg.HEADLESS,
            slow_mo=cfg.SLOW_MO,
            args=[
                "--disable-blink-features=AutomationControlled",
                "--no-sandbox",
                "--disable-infobars",
                "--disable-dev-shm-usage",
                "--lang=zh-CN,zh",
            ],
        )

    async def stop(self):
        for name in list(self._contexts):
            await self.close_context(name)
        if self._browser:
            await self._browser.close()
        if self._pw:
            await self._pw.stop()

    # ── 上下文管理 ────────────────────────────────────

    async def get_context(
        self, name: str, *, mobile: bool = False
    ) -> BrowserContext:
        """获取或创建一个命名的浏览器上下文（带 Cookie 持久化）"""
        if name in self._contexts:
            return self._contexts[name]

        cookie_file = cfg.COOKIES_DIR / f"{name}_cookies.json"
        viewport = cfg.MOBILE_VIEWPORT if mobile else cfg.DESKTOP_VIEWPORT

        ctx = await self._browser.new_context(
            viewport=viewport,
            user_agent=self._user_agent(mobile),
            locale="zh-CN",
            timezone_id="Asia/Shanghai",
            permissions=["geolocation"],
            geolocation={"latitude": 31.2304, "longitude": 121.4737},
            java_script_enabled=True,
            bypass_csp=True,
            ignore_https_errors=True,
        )

        # 反检测脚本
        await ctx.add_init_script(script=_STEALTH_JS)

        # 恢复已有 Cookie
        if cookie_file.exists():
            cookies = json.loads(cookie_file.read_text("utf-8"))
            await ctx.add_cookies(cookies)

        self._contexts[name] = ctx
        return ctx

    async def save_cookies(self, name: str):
        """持久化指定上下文的 Cookie"""
        ctx = self._contexts.get(name)
        if not ctx:
            return
        cookies = await ctx.cookies()
        cookie_file = cfg.COOKIES_DIR / f"{name}_cookies.json"
        cookie_file.write_text(
            json.dumps(cookies, ensure_ascii=False, indent=2), encoding="utf-8"
        )

    async def close_context(self, name: str):
        ctx = self._contexts.pop(name, None)
        if ctx:
            await self.save_cookies(name)
            await ctx.close()

    async def new_page(self, name: str, *, mobile: bool = False) -> Page:
        ctx = await self.get_context(name, mobile=mobile)
        page = await ctx.new_page()
        page.set_default_timeout(cfg.REQUEST_TIMEOUT)
        return page

    # ── 工具方法 ──────────────────────────────────────

    @staticmethod
    def _user_agent(mobile: bool) -> str:
        if mobile:
            return (
                "Mozilla/5.0 (iPhone; CPU iPhone OS 17_4 like Mac OS X) "
                "AppleWebKit/605.1.15 (KHTML, like Gecko) "
                "Version/17.4 Mobile/15E148 Safari/604.1"
            )
        return (
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) "
            "AppleWebKit/537.36 (KHTML, like Gecko) "
            "Chrome/124.0.0.0 Safari/537.36"
        )


async def random_sleep(low: float = 1.0, high: float = 3.0):
    """模拟人工的随机等待"""
    await asyncio.sleep(random.uniform(low, high))


async def smooth_scroll(page: Page, distance: int = 800, steps: int = 8):
    """模拟平滑滚动"""
    step = distance // steps
    for _ in range(steps):
        await page.mouse.wheel(0, step)
        await asyncio.sleep(random.uniform(0.05, 0.15))
    await random_sleep(0.5, 1.5)


async def wait_for_login(page: Page, check_selector: str, platform: str):
    """等待用户手动完成登录"""
    from rich.console import Console
    console = Console()

    console.print(
        f"\n[bold yellow]⚠ 请在浏览器中完成 {platform} 登录[/bold yellow]"
    )
    console.print("[dim]登录完成后程序将自动继续...[/dim]\n")

    for attempt in range(300):  # 最多等 5 分钟
        try:
            el = await page.query_selector(check_selector)
            if el:
                console.print(f"[bold green]✓ {platform} 登录成功[/bold green]\n")
                return True
        except Exception:
            pass
        await asyncio.sleep(1)

    console.print(f"[bold red]✗ {platform} 登录超时[/bold red]\n")
    return False


# ── 反检测 JS ─────────────────────────────────────────
_STEALTH_JS = """
// 隐藏 webdriver 标志
Object.defineProperty(navigator, 'webdriver', { get: () => undefined });

// 伪造 plugins
Object.defineProperty(navigator, 'plugins', {
    get: () => [1, 2, 3, 4, 5].map(() => ({
        name: 'Chrome PDF Plugin',
        description: 'Portable Document Format',
        filename: 'internal-pdf-viewer',
        length: 1,
    })),
});

// 伪造 languages
Object.defineProperty(navigator, 'languages', {
    get: () => ['zh-CN', 'zh', 'en-US', 'en'],
});

// 隐藏 HeadlessChrome
if (navigator.userAgent.includes('HeadlessChrome')) {
    Object.defineProperty(navigator, 'userAgent', {
        get: () => navigator.userAgent.replace('HeadlessChrome', 'Chrome'),
    });
}

// 防止 iframe 检测
const originalQuery = window.navigator.permissions.query;
window.navigator.permissions.query = (parameters) =>
    parameters.name === 'notifications'
        ? Promise.resolve({ state: Notification.permission })
        : originalQuery(parameters);

// Chrome runtime 伪造
window.chrome = { runtime: {}, loadTimes: () => {}, csi: () => {} };

// WebGL 供应商伪造
const getParameter = WebGLRenderingContext.prototype.getParameter;
WebGLRenderingContext.prototype.getParameter = function(parameter) {
    if (parameter === 37445) return 'Intel Inc.';
    if (parameter === 37446) return 'Intel Iris OpenGL Engine';
    return getParameter.call(this, parameter);
};
"""
