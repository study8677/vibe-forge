// ─── Base64 编解码 ───
App.registerTool({
    id: 'base64', name: 'Base64 编解码', desc: 'Base64 编码与解码转换',
    icon: 'B64', category: 'encoding',
    render() {
        return '<div class="tool-row"><div class="tool-col">' +
            '<label class="tool-label">输入</label>' +
            '<textarea class="tool-input" id="b64-in" placeholder="输入文本或 Base64 字符串..."></textarea>' +
            '</div><div class="tool-col">' +
            '<label class="tool-label">输出</label>' +
            '<textarea class="tool-output" id="b64-out" readonly></textarea>' +
            '</div></div>' +
            '<div class="btn-group">' +
            '<button class="btn btn-primary" id="b64-enc">编码</button>' +
            '<button class="btn btn-primary" id="b64-dec">解码</button>' +
            '<button class="btn" id="b64-copy">复制结果</button>' +
            '<button class="btn" id="b64-swap">交换</button>' +
            '<button class="btn" id="b64-clear">清空</button>' +
            '</div>';
    },
    init() {
        const $i = document.getElementById('b64-in'), $o = document.getElementById('b64-out');
        document.getElementById('b64-enc').onclick = () => {
            try {
                const bytes = new TextEncoder().encode($i.value);
                let bin = '';
                bytes.forEach(b => bin += String.fromCharCode(b));
                $o.value = btoa(bin);
            } catch (e) { $o.value = '编码错误: ' + e.message; }
        };
        document.getElementById('b64-dec').onclick = () => {
            try {
                const bin = atob($i.value.trim());
                const bytes = Uint8Array.from(bin, c => c.charCodeAt(0));
                $o.value = new TextDecoder().decode(bytes);
            } catch (e) { $o.value = '解码错误: ' + e.message; }
        };
        document.getElementById('b64-copy').onclick = () => App.copyToClipboard($o.value);
        document.getElementById('b64-swap').onclick = () => { const t = $i.value; $i.value = $o.value; $o.value = t; };
        document.getElementById('b64-clear').onclick = () => { $i.value = ''; $o.value = ''; };
    }
});

// ─── URL 编解码 ───
App.registerTool({
    id: 'url-encode', name: 'URL 编解码', desc: 'URL 编码与解码转换',
    icon: 'URL', category: 'encoding',
    render() {
        return '<div class="tool-row"><div class="tool-col">' +
            '<label class="tool-label">输入</label>' +
            '<textarea class="tool-input" id="url-in" placeholder="输入文本或 URL 编码字符串..."></textarea>' +
            '</div><div class="tool-col">' +
            '<label class="tool-label">输出</label>' +
            '<textarea class="tool-output" id="url-out" readonly></textarea>' +
            '</div></div>' +
            '<div class="tool-options">' +
            '<label class="tool-option"><input type="checkbox" class="tool-checkbox" id="url-full"> 全 URL 编码 (encodeURI)</label>' +
            '</div>' +
            '<div class="btn-group">' +
            '<button class="btn btn-primary" id="url-enc">编码</button>' +
            '<button class="btn btn-primary" id="url-dec">解码</button>' +
            '<button class="btn" id="url-copy">复制结果</button>' +
            '<button class="btn" id="url-clear">清空</button>' +
            '</div>';
    },
    init() {
        const $i = document.getElementById('url-in'), $o = document.getElementById('url-out');
        const $full = document.getElementById('url-full');
        document.getElementById('url-enc').onclick = () => {
            try { $o.value = $full.checked ? encodeURI($i.value) : encodeURIComponent($i.value); }
            catch (e) { $o.value = '编码错误: ' + e.message; }
        };
        document.getElementById('url-dec').onclick = () => {
            try { $o.value = $full.checked ? decodeURI($i.value.trim()) : decodeURIComponent($i.value.trim()); }
            catch (e) { $o.value = '解码错误: ' + e.message; }
        };
        document.getElementById('url-copy').onclick = () => App.copyToClipboard($o.value);
        document.getElementById('url-clear').onclick = () => { $i.value = ''; $o.value = ''; };
    }
});

// ─── HTML 实体编解码 ───
App.registerTool({
    id: 'html-entity', name: 'HTML 实体', desc: 'HTML 实体编码与解码',
    icon: '&lt;/&gt;', category: 'encoding',
    render() {
        return '<div class="tool-row"><div class="tool-col">' +
            '<label class="tool-label">输入</label>' +
            '<textarea class="tool-input" id="html-in" placeholder="输入 HTML 或实体编码文本..."></textarea>' +
            '</div><div class="tool-col">' +
            '<label class="tool-label">输出</label>' +
            '<textarea class="tool-output" id="html-out" readonly></textarea>' +
            '</div></div>' +
            '<div class="btn-group">' +
            '<button class="btn btn-primary" id="html-enc">编码</button>' +
            '<button class="btn btn-primary" id="html-dec">解码</button>' +
            '<button class="btn" id="html-copy">复制结果</button>' +
            '<button class="btn" id="html-clear">清空</button>' +
            '</div>';
    },
    init() {
        const $i = document.getElementById('html-in'), $o = document.getElementById('html-out');
        const entityMap = { '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;', '/': '&#x2F;' };
        document.getElementById('html-enc').onclick = () => {
            $o.value = $i.value.replace(/[&<>"'/]/g, c => entityMap[c]);
        };
        document.getElementById('html-dec').onclick = () => {
            const ta = document.createElement('textarea');
            ta.innerHTML = $i.value;
            $o.value = ta.value;
        };
        document.getElementById('html-copy').onclick = () => App.copyToClipboard($o.value);
        document.getElementById('html-clear').onclick = () => { $i.value = ''; $o.value = ''; };
    }
});

// ─── Unicode 编解码 ───
App.registerTool({
    id: 'unicode', name: 'Unicode 编解码', desc: 'Unicode 转义序列编解码',
    icon: 'U+', category: 'encoding',
    render() {
        return '<div class="tool-row"><div class="tool-col">' +
            '<label class="tool-label">输入</label>' +
            '<textarea class="tool-input" id="uni-in" placeholder="输入文本或 \\uXXXX 序列..."></textarea>' +
            '</div><div class="tool-col">' +
            '<label class="tool-label">输出</label>' +
            '<textarea class="tool-output" id="uni-out" readonly></textarea>' +
            '</div></div>' +
            '<div class="btn-group">' +
            '<button class="btn btn-primary" id="uni-enc">编码</button>' +
            '<button class="btn btn-primary" id="uni-dec">解码</button>' +
            '<button class="btn" id="uni-copy">复制结果</button>' +
            '<button class="btn" id="uni-clear">清空</button>' +
            '</div>';
    },
    init() {
        const $i = document.getElementById('uni-in'), $o = document.getElementById('uni-out');
        document.getElementById('uni-enc').onclick = () => {
            $o.value = Array.from($i.value).map(c => {
                const code = c.codePointAt(0);
                if (code > 0xFFFF) return '\\u{' + code.toString(16).toUpperCase() + '}';
                return '\\u' + code.toString(16).toUpperCase().padStart(4, '0');
            }).join('');
        };
        document.getElementById('uni-dec').onclick = () => {
            try {
                $o.value = $i.value.replace(/\\u\{([0-9a-fA-F]+)\}|\\u([0-9a-fA-F]{4})/g,
                    (_, p1, p2) => String.fromCodePoint(parseInt(p1 || p2, 16)));
            } catch (e) { $o.value = '解码错误: ' + e.message; }
        };
        document.getElementById('uni-copy').onclick = () => App.copyToClipboard($o.value);
        document.getElementById('uni-clear').onclick = () => { $i.value = ''; $o.value = ''; };
    }
});

// ─── Hex 编解码 ───
App.registerTool({
    id: 'hex', name: 'Hex 编解码', desc: '文本与十六进制互转',
    icon: '0x', category: 'encoding',
    render() {
        return '<div class="tool-row"><div class="tool-col">' +
            '<label class="tool-label">输入</label>' +
            '<textarea class="tool-input" id="hex-in" placeholder="输入文本或十六进制字符串..."></textarea>' +
            '</div><div class="tool-col">' +
            '<label class="tool-label">输出</label>' +
            '<textarea class="tool-output" id="hex-out" readonly></textarea>' +
            '</div></div>' +
            '<div class="tool-options">' +
            '<label class="tool-option"><input type="checkbox" class="tool-checkbox" id="hex-space" checked> 字节间添加空格</label>' +
            '<label class="tool-option"><input type="checkbox" class="tool-checkbox" id="hex-upper"> 大写</label>' +
            '</div>' +
            '<div class="btn-group">' +
            '<button class="btn btn-primary" id="hex-enc">文本 → Hex</button>' +
            '<button class="btn btn-primary" id="hex-dec">Hex → 文本</button>' +
            '<button class="btn" id="hex-copy">复制结果</button>' +
            '<button class="btn" id="hex-clear">清空</button>' +
            '</div>';
    },
    init() {
        const $i = document.getElementById('hex-in'), $o = document.getElementById('hex-out');
        document.getElementById('hex-enc').onclick = () => {
            const bytes = new TextEncoder().encode($i.value);
            const sep = document.getElementById('hex-space').checked ? ' ' : '';
            const up = document.getElementById('hex-upper').checked;
            let hex = Array.from(bytes).map(b => b.toString(16).padStart(2, '0')).join(sep);
            $o.value = up ? hex.toUpperCase() : hex;
        };
        document.getElementById('hex-dec').onclick = () => {
            try {
                const cleaned = $i.value.replace(/\s+/g, '').replace(/^0x/i, '');
                if (cleaned.length % 2 !== 0) throw new Error('十六进制长度必须为偶数');
                const bytes = new Uint8Array(cleaned.length / 2);
                for (let i = 0; i < cleaned.length; i += 2) {
                    bytes[i / 2] = parseInt(cleaned.substr(i, 2), 16);
                }
                $o.value = new TextDecoder().decode(bytes);
            } catch (e) { $o.value = '解码错误: ' + e.message; }
        };
        document.getElementById('hex-copy').onclick = () => App.copyToClipboard($o.value);
        document.getElementById('hex-clear').onclick = () => { $i.value = ''; $o.value = ''; };
    }
});
