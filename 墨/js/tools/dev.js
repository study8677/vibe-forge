// ─── UUID 生成器 ───
App.registerTool({
    id: 'uuid', name: 'UUID 生成器', desc: 'UUID / GUID 批量生成',
    icon: 'UID', category: 'dev',
    render() {
        return '<div class="tool-options">' +
            '<label class="tool-option">数量 <input type="number" class="tool-text-input" id="uuid-count" value="5" min="1" max="100" style="width:80px"></label>' +
            '<label class="tool-option"><input type="checkbox" class="tool-checkbox" id="uuid-upper"> 大写</label>' +
            '<label class="tool-option"><input type="checkbox" class="tool-checkbox" id="uuid-nohyphen"> 无连字符</label>' +
            '<label class="tool-option">版本 <select class="tool-select" id="uuid-ver">' +
            '<option value="v4" selected>v4 (随机)</option>' +
            '<option value="nil">NIL (全零)</option>' +
            '</select></label>' +
            '</div>' +
            '<div class="btn-group" style="margin-bottom:16px">' +
            '<button class="btn btn-primary" id="uuid-gen">生成</button>' +
            '<button class="btn" id="uuid-copy">复制全部</button>' +
            '</div>' +
            '<div class="tool-section">' +
            '<label class="tool-label">结果</label>' +
            '<textarea class="tool-output" id="uuid-out" readonly style="min-height:250px"></textarea>' +
            '</div>';
    },
    init() {
        function uuidv4() {
            if (crypto.randomUUID) return crypto.randomUUID();
            return '10000000-1000-4000-8000-100000000000'.replace(/[018]/g, c =>
                (+c ^ crypto.getRandomValues(new Uint8Array(1))[0] & 15 >> +c / 4).toString(16)
            );
        }
        document.getElementById('uuid-gen').onclick = () => {
            const count = Math.min(parseInt(document.getElementById('uuid-count').value) || 1, 100);
            const upper = document.getElementById('uuid-upper').checked;
            const noHyphen = document.getElementById('uuid-nohyphen').checked;
            const ver = document.getElementById('uuid-ver').value;
            const uuids = [];
            for (let i = 0; i < count; i++) {
                let id = ver === 'nil' ? '00000000-0000-0000-0000-000000000000' : uuidv4();
                if (noHyphen) id = id.replace(/-/g, '');
                if (upper) id = id.toUpperCase();
                uuids.push(id);
            }
            document.getElementById('uuid-out').value = uuids.join('\n');
        };
        document.getElementById('uuid-copy').onclick = () => App.copyToClipboard(document.getElementById('uuid-out').value);
        // Generate on load
        document.getElementById('uuid-gen').click();
    }
});

// ─── 时间戳转换 ───
App.registerTool({
    id: 'timestamp', name: '时间戳转换', desc: '时间戳与日期互转',
    icon: 'T⏱', category: 'dev',
    render() {
        return '<div class="tool-section">' +
            '<label class="tool-label">当前时间戳</label>' +
            '<div class="tool-row" style="gap:8px;align-items:center">' +
            '<input type="text" class="tool-text-input" id="ts-now" readonly style="flex:1">' +
            '<button class="btn btn-sm" id="ts-copy-now">复制</button>' +
            '<button class="btn btn-sm" id="ts-refresh">刷新</button>' +
            '</div></div>' +
            '<div class="tool-section" style="margin-top:20px">' +
            '<label class="tool-label">时间戳 → 日期</label>' +
            '<div class="tool-row" style="gap:8px">' +
            '<input type="text" class="tool-text-input" id="ts-input" placeholder="输入时间戳 (秒或毫秒)" style="flex:1">' +
            '<button class="btn btn-primary" id="ts-to-date">转换</button>' +
            '</div>' +
            '<div class="img-info" id="ts-result" style="margin-top:8px"></div>' +
            '</div>' +
            '<div class="tool-section" style="margin-top:20px">' +
            '<label class="tool-label">日期 → 时间戳</label>' +
            '<div class="tool-row" style="gap:8px">' +
            '<input type="datetime-local" class="tool-text-input" id="ts-date" style="flex:1">' +
            '<button class="btn btn-primary" id="ts-to-stamp">转换</button>' +
            '</div>' +
            '<div class="tool-row" style="gap:8px;margin-top:8px">' +
            '<input type="text" class="tool-text-input" id="ts-stamp-result" readonly placeholder="时间戳" style="flex:1">' +
            '<button class="btn btn-sm" id="ts-copy-stamp">复制</button>' +
            '</div></div>';
    },
    init() {
        const showNow = () => {
            const now = Date.now();
            document.getElementById('ts-now').value = Math.floor(now / 1000) + ' (秒) | ' + now + ' (毫秒)';
        };
        showNow();
        const timer = setInterval(showNow, 1000);

        // Set default datetime to now
        const now = new Date();
        now.setMinutes(now.getMinutes() - now.getTimezoneOffset());
        document.getElementById('ts-date').value = now.toISOString().slice(0, 16);

        document.getElementById('ts-copy-now').onclick = () => App.copyToClipboard(String(Math.floor(Date.now() / 1000)));
        document.getElementById('ts-refresh').onclick = showNow;

        document.getElementById('ts-to-date').onclick = () => {
            let v = document.getElementById('ts-input').value.trim();
            if (!v) return;
            let ts = parseInt(v);
            if (String(ts).length <= 10) ts *= 1000; // seconds to ms
            const d = new Date(ts);
            if (isNaN(d.getTime())) { document.getElementById('ts-result').textContent = '无效时间戳'; return; }
            document.getElementById('ts-result').innerHTML =
                '本地时间: ' + d.toLocaleString() + '<br>' +
                'UTC 时间: ' + d.toUTCString() + '<br>' +
                'ISO 8601: ' + d.toISOString() + '<br>' +
                '相对时间: ' + relativeTime(d);
        };

        document.getElementById('ts-to-stamp').onclick = () => {
            const v = document.getElementById('ts-date').value;
            if (!v) return;
            const d = new Date(v);
            document.getElementById('ts-stamp-result').value = Math.floor(d.getTime() / 1000) + ' (秒) | ' + d.getTime() + ' (毫秒)';
        };
        document.getElementById('ts-copy-stamp').onclick = () => {
            const v = document.getElementById('ts-stamp-result').value.split(' ')[0];
            App.copyToClipboard(v);
        };

        function relativeTime(d) {
            const diff = Date.now() - d.getTime();
            const abs = Math.abs(diff);
            const suffix = diff > 0 ? '前' : '后';
            if (abs < 60000) return Math.floor(abs / 1000) + ' 秒' + suffix;
            if (abs < 3600000) return Math.floor(abs / 60000) + ' 分钟' + suffix;
            if (abs < 86400000) return Math.floor(abs / 3600000) + ' 小时' + suffix;
            return Math.floor(abs / 86400000) + ' 天' + suffix;
        }

        // Cleanup timer when navigating away
        const origHash = window.location.hash;
        const check = () => {
            if (window.location.hash !== origHash) { clearInterval(timer); window.removeEventListener('hashchange', check); }
        };
        window.addEventListener('hashchange', check);
    }
});

// ─── 颜色转换 ───
App.registerTool({
    id: 'color', name: '颜色转换', desc: 'HEX / RGB / HSL 互转',
    icon: '#C', category: 'dev',
    render() {
        return '<div class="tool-row"><div class="tool-col">' +
            '<div class="tool-section">' +
            '<label class="tool-label">颜色选择器</label>' +
            '<input type="color" id="color-picker" value="#3498db" style="width:100%;height:60px;border:1px solid var(--border);background:var(--bg-1);cursor:pointer;border-radius:var(--radius)">' +
            '</div>' +
            '<div class="tool-section">' +
            '<label class="tool-label">HEX</label>' +
            '<input type="text" class="tool-text-input" id="color-hex" value="#3498db" placeholder="#RRGGBB">' +
            '</div>' +
            '<div class="tool-section">' +
            '<label class="tool-label">RGB</label>' +
            '<input type="text" class="tool-text-input" id="color-rgb" placeholder="rgb(r, g, b)">' +
            '</div>' +
            '<div class="tool-section">' +
            '<label class="tool-label">HSL</label>' +
            '<input type="text" class="tool-text-input" id="color-hsl" placeholder="hsl(h, s%, l%)">' +
            '</div>' +
            '<div class="tool-section">' +
            '<label class="tool-label">RGBA (CSS)</label>' +
            '<input type="text" class="tool-text-input" id="color-rgba" placeholder="rgba(r, g, b, a)">' +
            '</div>' +
            '</div><div class="tool-col">' +
            '<div class="tool-section">' +
            '<label class="tool-label">预览</label>' +
            '<div class="color-box" id="color-preview" style="width:100%;height:120px;background:#3498db"></div>' +
            '</div>' +
            '<div class="tool-section">' +
            '<label class="tool-label">色值分量</label>' +
            '<div class="img-info" id="color-detail"></div>' +
            '</div>' +
            '<div class="btn-group">' +
            '<button class="btn btn-sm" id="color-copy-hex">复制 HEX</button>' +
            '<button class="btn btn-sm" id="color-copy-rgb">复制 RGB</button>' +
            '<button class="btn btn-sm" id="color-copy-hsl">复制 HSL</button>' +
            '</div>' +
            '</div></div>';
    },
    init() {
        function hexToRgb(hex) {
            hex = hex.replace('#', '');
            if (hex.length === 3) hex = hex[0] + hex[0] + hex[1] + hex[1] + hex[2] + hex[2];
            return { r: parseInt(hex.substr(0, 2), 16), g: parseInt(hex.substr(2, 2), 16), b: parseInt(hex.substr(4, 2), 16) };
        }
        function rgbToHsl(r, g, b) {
            r /= 255; g /= 255; b /= 255;
            const max = Math.max(r, g, b), min = Math.min(r, g, b);
            let h, s, l = (max + min) / 2;
            if (max === min) { h = s = 0; } else {
                const d = max - min;
                s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
                switch (max) {
                    case r: h = ((g - b) / d + (g < b ? 6 : 0)) / 6; break;
                    case g: h = ((b - r) / d + 2) / 6; break;
                    case b: h = ((r - g) / d + 4) / 6; break;
                }
            }
            return { h: Math.round(h * 360), s: Math.round(s * 100), l: Math.round(l * 100) };
        }
        function hslToRgb(h, s, l) {
            h /= 360; s /= 100; l /= 100;
            let r, g, b;
            if (s === 0) { r = g = b = l; } else {
                const hue2rgb = (p, q, t) => {
                    if (t < 0) t += 1;
                    if (t > 1) t -= 1;
                    if (t < 1 / 6) return p + (q - p) * 6 * t;
                    if (t < 1 / 2) return q;
                    if (t < 2 / 3) return p + (q - p) * (2 / 3 - t) * 6;
                    return p;
                };
                const q = l < 0.5 ? l * (1 + s) : l + s - l * s;
                const p = 2 * l - q;
                r = hue2rgb(p, q, h + 1 / 3);
                g = hue2rgb(p, q, h);
                b = hue2rgb(p, q, h - 1 / 3);
            }
            return { r: Math.round(r * 255), g: Math.round(g * 255), b: Math.round(b * 255) };
        }
        function rgbToHex(r, g, b) {
            return '#' + [r, g, b].map(c => c.toString(16).padStart(2, '0')).join('');
        }

        function updateFromHex(hex) {
            const rgb = hexToRgb(hex);
            const hsl = rgbToHsl(rgb.r, rgb.g, rgb.b);
            document.getElementById('color-hex').value = hex;
            document.getElementById('color-rgb').value = 'rgb(' + rgb.r + ', ' + rgb.g + ', ' + rgb.b + ')';
            document.getElementById('color-hsl').value = 'hsl(' + hsl.h + ', ' + hsl.s + '%, ' + hsl.l + '%)';
            document.getElementById('color-rgba').value = 'rgba(' + rgb.r + ', ' + rgb.g + ', ' + rgb.b + ', 1)';
            document.getElementById('color-preview').style.background = hex;
            document.getElementById('color-picker').value = hex;
            document.getElementById('color-detail').innerHTML =
                'R: ' + rgb.r + ' | G: ' + rgb.g + ' | B: ' + rgb.b + '<br>' +
                'H: ' + hsl.h + '° | S: ' + hsl.s + '% | L: ' + hsl.l + '%';
        }

        updateFromHex('#3498db');

        document.getElementById('color-picker').oninput = (e) => updateFromHex(e.target.value);
        document.getElementById('color-hex').onchange = (e) => {
            let v = e.target.value.trim();
            if (!v.startsWith('#')) v = '#' + v;
            if (/^#[0-9a-fA-F]{3,6}$/.test(v)) updateFromHex(v);
        };
        document.getElementById('color-rgb').onchange = (e) => {
            const m = e.target.value.match(/(\d+)\s*,\s*(\d+)\s*,\s*(\d+)/);
            if (m) updateFromHex(rgbToHex(+m[1], +m[2], +m[3]));
        };
        document.getElementById('color-hsl').onchange = (e) => {
            const m = e.target.value.match(/(\d+)\s*,\s*(\d+)%?\s*,\s*(\d+)/);
            if (m) {
                const rgb = hslToRgb(+m[1], +m[2], +m[3]);
                updateFromHex(rgbToHex(rgb.r, rgb.g, rgb.b));
            }
        };

        document.getElementById('color-copy-hex').onclick = () => App.copyToClipboard(document.getElementById('color-hex').value);
        document.getElementById('color-copy-rgb').onclick = () => App.copyToClipboard(document.getElementById('color-rgb').value);
        document.getElementById('color-copy-hsl').onclick = () => App.copyToClipboard(document.getElementById('color-hsl').value);
    }
});

// ─── 进制转换 ───
App.registerTool({
    id: 'radix', name: '进制转换', desc: '2 / 8 / 10 / 16 进制互转',
    icon: 'Bin', category: 'dev',
    render() {
        return '<div class="tool-section">' +
            '<label class="tool-label">输入数值</label>' +
            '<input type="text" class="tool-text-input" id="radix-in" placeholder="输入数值..." value="255">' +
            '</div>' +
            '<div class="tool-options">' +
            '<label class="tool-option">输入进制 <select class="tool-select" id="radix-from">' +
            '<option value="2">二进制</option>' +
            '<option value="8">八进制</option>' +
            '<option value="10" selected>十进制</option>' +
            '<option value="16">十六进制</option>' +
            '<option value="custom">自定义</option>' +
            '</select></label>' +
            '<input type="number" class="tool-text-input" id="radix-custom-from" value="10" min="2" max="36" style="width:70px;display:none">' +
            '</div>' +
            '<div class="btn-group" style="margin-bottom:20px">' +
            '<button class="btn btn-primary" id="radix-calc">转换</button>' +
            '</div>' +
            '<div class="stat-grid">' +
            '<div class="stat-box"><div class="stat-value" id="radix-bin" style="font-size:16px;word-break:break-all">-</div><div class="stat-label">二进制 (BIN)</div></div>' +
            '<div class="stat-box"><div class="stat-value" id="radix-oct" style="font-size:20px">-</div><div class="stat-label">八进制 (OCT)</div></div>' +
            '<div class="stat-box"><div class="stat-value" id="radix-dec" style="font-size:20px">-</div><div class="stat-label">十进制 (DEC)</div></div>' +
            '<div class="stat-box"><div class="stat-value" id="radix-hex" style="font-size:20px">-</div><div class="stat-label">十六进制 (HEX)</div></div>' +
            '</div>' +
            '<div class="btn-group" style="margin-top:12px">' +
            '<button class="btn btn-sm" data-copy="radix-bin">复制 BIN</button>' +
            '<button class="btn btn-sm" data-copy="radix-oct">复制 OCT</button>' +
            '<button class="btn btn-sm" data-copy="radix-dec">复制 DEC</button>' +
            '<button class="btn btn-sm" data-copy="radix-hex">复制 HEX</button>' +
            '</div>';
    },
    init() {
        document.getElementById('radix-from').onchange = (e) => {
            document.getElementById('radix-custom-from').style.display = e.target.value === 'custom' ? '' : 'none';
        };
        const calc = () => {
            const input = document.getElementById('radix-in').value.trim();
            let base = parseInt(document.getElementById('radix-from').value);
            if (isNaN(base)) base = parseInt(document.getElementById('radix-custom-from').value) || 10;
            try {
                const num = parseInt(input, base);
                if (isNaN(num)) throw new Error('无效数值');
                document.getElementById('radix-bin').textContent = num.toString(2);
                document.getElementById('radix-oct').textContent = num.toString(8);
                document.getElementById('radix-dec').textContent = num.toString(10);
                document.getElementById('radix-hex').textContent = num.toString(16).toUpperCase();
            } catch (e) {
                ['radix-bin', 'radix-oct', 'radix-dec', 'radix-hex'].forEach(id => {
                    document.getElementById(id).textContent = '错误';
                });
            }
        };
        document.getElementById('radix-calc').onclick = calc;
        document.getElementById('radix-in').addEventListener('keyup', (e) => { if (e.key === 'Enter') calc(); });

        // Copy buttons
        document.querySelectorAll('[data-copy]').forEach(btn => {
            btn.onclick = () => App.copyToClipboard(document.getElementById(btn.dataset.copy).textContent);
        });

        // Initial calc
        calc();
    }
});
