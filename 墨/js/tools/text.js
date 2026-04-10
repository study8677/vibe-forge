// ─── JSON 格式化 ───
App.registerTool({
    id: 'json', name: 'JSON 格式化', desc: 'JSON 格式化与校验',
    icon: '{ }', category: 'text',
    render() {
        return '<div class="tool-row"><div class="tool-col">' +
            '<label class="tool-label">输入 JSON</label>' +
            '<textarea class="tool-input" id="json-in" placeholder=\'{"key": "value"}\'></textarea>' +
            '</div><div class="tool-col">' +
            '<label class="tool-label">输出</label>' +
            '<textarea class="tool-output" id="json-out" readonly></textarea>' +
            '</div></div>' +
            '<div class="tool-options">' +
            '<label class="tool-option">缩进 <select class="tool-select" id="json-indent">' +
            '<option value="2" selected>2 空格</option>' +
            '<option value="4">4 空格</option>' +
            '<option value="tab">Tab</option>' +
            '<option value="0">压缩</option>' +
            '</select></label>' +
            '<label class="tool-option"><input type="checkbox" class="tool-checkbox" id="json-sort"> 排序键名</label>' +
            '</div>' +
            '<div class="btn-group">' +
            '<button class="btn btn-primary" id="json-fmt">格式化</button>' +
            '<button class="btn btn-primary" id="json-min">压缩</button>' +
            '<button class="btn" id="json-copy">复制</button>' +
            '<button class="btn" id="json-clear">清空</button>' +
            '</div>';
    },
    init() {
        function sortKeys(obj) {
            if (Array.isArray(obj)) return obj.map(sortKeys);
            if (obj && typeof obj === 'object') {
                return Object.keys(obj).sort().reduce((a, k) => { a[k] = sortKeys(obj[k]); return a; }, {});
            }
            return obj;
        }
        document.getElementById('json-fmt').onclick = () => {
            try {
                let obj = JSON.parse(document.getElementById('json-in').value);
                if (document.getElementById('json-sort').checked) obj = sortKeys(obj);
                const ind = document.getElementById('json-indent').value;
                const indent = ind === 'tab' ? '\t' : parseInt(ind);
                document.getElementById('json-out').value = JSON.stringify(obj, null, indent);
            } catch (e) { document.getElementById('json-out').value = 'JSON 解析错误: ' + e.message; }
        };
        document.getElementById('json-min').onclick = () => {
            try {
                const obj = JSON.parse(document.getElementById('json-in').value);
                document.getElementById('json-out').value = JSON.stringify(obj);
            } catch (e) { document.getElementById('json-out').value = 'JSON 解析错误: ' + e.message; }
        };
        document.getElementById('json-copy').onclick = () => App.copyToClipboard(document.getElementById('json-out').value);
        document.getElementById('json-clear').onclick = () => {
            document.getElementById('json-in').value = '';
            document.getElementById('json-out').value = '';
        };
    }
});

// ─── XML 格式化 ───
App.registerTool({
    id: 'xml', name: 'XML 格式化', desc: 'XML 格式化与压缩',
    icon: 'XML', category: 'text',
    render() {
        return '<div class="tool-row"><div class="tool-col">' +
            '<label class="tool-label">输入 XML</label>' +
            '<textarea class="tool-input" id="xml-in" placeholder="<root><item>text</item></root>"></textarea>' +
            '</div><div class="tool-col">' +
            '<label class="tool-label">输出</label>' +
            '<textarea class="tool-output" id="xml-out" readonly></textarea>' +
            '</div></div>' +
            '<div class="btn-group">' +
            '<button class="btn btn-primary" id="xml-fmt">格式化</button>' +
            '<button class="btn btn-primary" id="xml-min">压缩</button>' +
            '<button class="btn" id="xml-copy">复制</button>' +
            '<button class="btn" id="xml-clear">清空</button>' +
            '</div>';
    },
    init() {
        function formatXml(xml) {
            let formatted = '';
            let indent = '';
            const tab = '  ';
            xml = xml.replace(/(>)\s*(<)/g, '$1\n$2');
            xml.split('\n').forEach(node => {
                node = node.trim();
                if (!node) return;
                if (node.match(/^<\/\w/)) indent = indent.substring(tab.length);
                formatted += indent + node + '\n';
                if (node.match(/^<\w([^>]*[^/])?>.*$/) && !node.match(/^<\w[^>]*\/>/)) {
                    if (!node.match(/<\/\w/)) indent += tab;
                }
            });
            return formatted.trim();
        }
        document.getElementById('xml-fmt').onclick = () => {
            try { document.getElementById('xml-out').value = formatXml(document.getElementById('xml-in').value); }
            catch (e) { document.getElementById('xml-out').value = '格式化错误: ' + e.message; }
        };
        document.getElementById('xml-min').onclick = () => {
            document.getElementById('xml-out').value = document.getElementById('xml-in').value
                .replace(/>\s+</g, '><').replace(/\s+/g, ' ').trim();
        };
        document.getElementById('xml-copy').onclick = () => App.copyToClipboard(document.getElementById('xml-out').value);
        document.getElementById('xml-clear').onclick = () => {
            document.getElementById('xml-in').value = '';
            document.getElementById('xml-out').value = '';
        };
    }
});

// ─── 正则测试器 ───
App.registerTool({
    id: 'regex', name: '正则测试', desc: '正则表达式在线测试',
    icon: '/./', category: 'text',
    render() {
        return '<div class="tool-section">' +
            '<label class="tool-label">正则表达式</label>' +
            '<div class="tool-row" style="gap:8px">' +
            '<input type="text" class="tool-text-input" id="regex-pattern" placeholder="输入正则表达式..." style="flex:1">' +
            '<input type="text" class="tool-text-input" id="regex-flags" placeholder="flags" value="g" style="width:80px">' +
            '</div></div>' +
            '<div class="tool-section">' +
            '<label class="tool-label">测试文本</label>' +
            '<textarea class="tool-input" id="regex-text" placeholder="输入测试文本..." style="min-height:150px"></textarea>' +
            '</div>' +
            '<div class="regex-info" id="regex-info"></div>' +
            '<div class="tool-section">' +
            '<label class="tool-label">匹配结果</label>' +
            '<div class="regex-result" id="regex-result">输入正则和文本后自动匹配</div>' +
            '</div>' +
            '<div class="tool-section">' +
            '<label class="tool-label">匹配列表</label>' +
            '<textarea class="tool-output" id="regex-matches" readonly style="min-height:100px"></textarea>' +
            '</div>';
    },
    init() {
        const run = () => {
            const pattern = document.getElementById('regex-pattern').value;
            const flags = document.getElementById('regex-flags').value;
            const text = document.getElementById('regex-text').value;
            if (!pattern || !text) {
                document.getElementById('regex-result').innerHTML = '<span style="color:var(--text-4)">输入正则和文本后自动匹配</span>';
                document.getElementById('regex-info').textContent = '';
                document.getElementById('regex-matches').value = '';
                return;
            }
            try {
                const re = new RegExp(pattern, flags);
                const matches = [];
                let m;
                const re2 = new RegExp(pattern, flags.includes('g') ? flags : flags + 'g');
                while ((m = re2.exec(text)) !== null) {
                    matches.push({ index: m.index, length: m[0].length, value: m[0], groups: m.slice(1) });
                    if (!m[0].length) re2.lastIndex++;
                }
                document.getElementById('regex-info').textContent = matches.length + ' 个匹配';

                // Highlight
                let highlighted = '';
                let last = 0;
                matches.forEach(match => {
                    highlighted += App.escapeHtml(text.slice(last, match.index));
                    highlighted += '<span class="regex-match">' + App.escapeHtml(match.value) + '</span>';
                    last = match.index + match.length;
                });
                highlighted += App.escapeHtml(text.slice(last));
                document.getElementById('regex-result').innerHTML = highlighted || App.escapeHtml(text);

                // Match list
                document.getElementById('regex-matches').value = matches.map((m, i) =>
                    '匹配 ' + (i + 1) + ': "' + m.value + '" (位置: ' + m.index + ')' +
                    (m.groups.length ? '\n  捕获组: ' + m.groups.map((g, j) => '$' + (j + 1) + '="' + (g || '') + '"').join(', ') : '')
                ).join('\n');
            } catch (e) {
                document.getElementById('regex-info').textContent = '正则错误: ' + e.message;
                document.getElementById('regex-result').innerHTML = '<span style="color:#a55">' + App.escapeHtml(e.message) + '</span>';
                document.getElementById('regex-matches').value = '';
            }
        };
        document.getElementById('regex-pattern').addEventListener('input', run);
        document.getElementById('regex-flags').addEventListener('input', run);
        document.getElementById('regex-text').addEventListener('input', run);
    }
});

// ─── 文本对比 ───
App.registerTool({
    id: 'diff', name: '文本对比', desc: '文本差异对比 (Diff)',
    icon: 'DIF', category: 'text',
    render() {
        return '<div class="tool-row"><div class="tool-col">' +
            '<label class="tool-label">文本 A (原始)</label>' +
            '<textarea class="tool-input" id="diff-a" placeholder="输入原始文本..."></textarea>' +
            '</div><div class="tool-col">' +
            '<label class="tool-label">文本 B (修改后)</label>' +
            '<textarea class="tool-input" id="diff-b" placeholder="输入修改后文本..."></textarea>' +
            '</div></div>' +
            '<div class="btn-group" style="margin-bottom:16px">' +
            '<button class="btn btn-primary" id="diff-run">对比</button>' +
            '<button class="btn" id="diff-clear">清空</button>' +
            '</div>' +
            '<div class="tool-section">' +
            '<label class="tool-label">对比结果</label>' +
            '<div class="diff-output" id="diff-out">点击「对比」查看差异</div>' +
            '</div>';
    },
    init() {
        function lcs(a, b) {
            const m = a.length, n = b.length;
            const dp = [];
            for (let i = 0; i <= m; i++) {
                dp[i] = [];
                for (let j = 0; j <= n; j++) dp[i][j] = 0;
            }
            for (let i = 1; i <= m; i++) {
                for (let j = 1; j <= n; j++) {
                    dp[i][j] = a[i - 1] === b[j - 1] ? dp[i - 1][j - 1] + 1 : Math.max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
            const result = [];
            let i = m, j = n;
            while (i > 0 || j > 0) {
                if (i > 0 && j > 0 && a[i - 1] === b[j - 1]) {
                    result.unshift({ type: 'same', text: a[i - 1] });
                    i--; j--;
                } else if (j > 0 && (i === 0 || dp[i][j - 1] >= dp[i - 1][j])) {
                    result.unshift({ type: 'add', text: b[j - 1] });
                    j--;
                } else {
                    result.unshift({ type: 'del', text: a[i - 1] });
                    i--;
                }
            }
            return result;
        }
        document.getElementById('diff-run').onclick = () => {
            const a = document.getElementById('diff-a').value.split('\n');
            const b = document.getElementById('diff-b').value.split('\n');
            const diff = lcs(a, b);
            let html = '';
            let lineA = 0, lineB = 0;
            diff.forEach(d => {
                const escaped = App.escapeHtml(d.text);
                if (d.type === 'same') {
                    lineA++; lineB++;
                    html += '<div class="diff-line diff-same">  ' + lineA + ' | ' + lineB + '  ' + escaped + '</div>';
                } else if (d.type === 'del') {
                    lineA++;
                    html += '<div class="diff-line diff-del">- ' + lineA + '        ' + escaped + '</div>';
                } else {
                    lineB++;
                    html += '<div class="diff-line diff-add">+      | ' + lineB + '  ' + escaped + '</div>';
                }
            });
            document.getElementById('diff-out').innerHTML = html || '<span style="color:var(--text-3)">两段文本完全相同</span>';
        };
        document.getElementById('diff-clear').onclick = () => {
            document.getElementById('diff-a').value = '';
            document.getElementById('diff-b').value = '';
            document.getElementById('diff-out').innerHTML = '';
        };
    }
});

// ─── 字数统计 ───
App.registerTool({
    id: 'wordcount', name: '字数统计', desc: '字符 / 词 / 行数统计',
    icon: 'Wc', category: 'text',
    render() {
        return '<div class="tool-section">' +
            '<label class="tool-label">输入文本</label>' +
            '<textarea class="tool-input" id="wc-in" placeholder="输入或粘贴文本，实时统计..." style="min-height:250px"></textarea>' +
            '</div>' +
            '<div class="stat-grid" id="wc-stats">' +
            '<div class="stat-box"><div class="stat-value" id="wc-chars">0</div><div class="stat-label">字符</div></div>' +
            '<div class="stat-box"><div class="stat-value" id="wc-chars-ns">0</div><div class="stat-label">不含空格</div></div>' +
            '<div class="stat-box"><div class="stat-value" id="wc-words">0</div><div class="stat-label">英文单词</div></div>' +
            '<div class="stat-box"><div class="stat-value" id="wc-cn">0</div><div class="stat-label">中文字符</div></div>' +
            '<div class="stat-box"><div class="stat-value" id="wc-lines">0</div><div class="stat-label">行数</div></div>' +
            '<div class="stat-box"><div class="stat-value" id="wc-paras">0</div><div class="stat-label">段落</div></div>' +
            '<div class="stat-box"><div class="stat-value" id="wc-bytes">0</div><div class="stat-label">UTF-8 字节</div></div>' +
            '<div class="stat-box"><div class="stat-value" id="wc-sents">0</div><div class="stat-label">句子</div></div>' +
            '</div>';
    },
    init() {
        const update = () => {
            const text = document.getElementById('wc-in').value;
            document.getElementById('wc-chars').textContent = text.length;
            document.getElementById('wc-chars-ns').textContent = text.replace(/\s/g, '').length;
            document.getElementById('wc-words').textContent = text.trim() ? (text.match(/[a-zA-Z]+/g) || []).length : 0;
            document.getElementById('wc-cn').textContent = (text.match(/[\u4e00-\u9fff]/g) || []).length;
            document.getElementById('wc-lines').textContent = text ? text.split('\n').length : 0;
            document.getElementById('wc-paras').textContent = text.trim() ? text.split(/\n\s*\n/).filter(p => p.trim()).length : 0;
            document.getElementById('wc-bytes').textContent = new TextEncoder().encode(text).length;
            document.getElementById('wc-sents').textContent = text.trim() ? (text.match(/[.!?。！？；]+/g) || []).length : 0;
        };
        document.getElementById('wc-in').addEventListener('input', update);
    }
});

// ─── Markdown 预览 ───
App.registerTool({
    id: 'markdown', name: 'Markdown 预览', desc: 'Markdown 实时渲染预览',
    icon: 'MD', category: 'text',
    render() {
        return '<div class="tool-row"><div class="tool-col">' +
            '<label class="tool-label">Markdown 源码</label>' +
            '<textarea class="tool-input" id="md-in" placeholder="输入 Markdown 文本..." style="min-height:400px">' +
            '# 标题\n\n## 二级标题\n\n这是一段 **加粗** 和 *斜体* 文本。\n\n' +
            '- 列表项 1\n- 列表项 2\n- 列表项 3\n\n' +
            '> 引用文本\n\n' +
            '```\nconsole.log("Hello World");\n```\n\n' +
            '[链接文本](https://example.com)\n\n---\n\n' +
            '| 表头1 | 表头2 |\n|-------|-------|\n| 数据1 | 数据2 |' +
            '</textarea>' +
            '</div><div class="tool-col">' +
            '<label class="tool-label">预览</label>' +
            '<div class="md-preview" id="md-out"></div>' +
            '</div></div>';
    },
    init() {
        function renderMd(md) {
            let html = md;
            // Code blocks
            html = html.replace(/```(\w*)\n([\s\S]*?)```/g, function(_, lang, code) {
                return '<pre><code>' + App.escapeHtml(code) + '</code></pre>';
            });
            // Inline code
            html = html.replace(/`([^`]+)`/g, '<code>$1</code>');
            // Tables
            html = html.replace(/^\|(.+)\|\s*\n\|[-| :]+\|\s*\n((?:\|.+\|\s*\n?)*)/gm, function(_, header, body) {
                let t = '<table><thead><tr>';
                header.split('|').filter(c => c.trim()).forEach(c => t += '<th>' + c.trim() + '</th>');
                t += '</tr></thead><tbody>';
                body.trim().split('\n').forEach(row => {
                    t += '<tr>';
                    row.split('|').filter(c => c.trim()).forEach(c => t += '<td>' + c.trim() + '</td>');
                    t += '</tr>';
                });
                return t + '</tbody></table>';
            });
            // Headers
            html = html.replace(/^######\s(.+)$/gm, '<h6>$1</h6>');
            html = html.replace(/^#####\s(.+)$/gm, '<h5>$1</h5>');
            html = html.replace(/^####\s(.+)$/gm, '<h4>$1</h4>');
            html = html.replace(/^###\s(.+)$/gm, '<h3>$1</h3>');
            html = html.replace(/^##\s(.+)$/gm, '<h2>$1</h2>');
            html = html.replace(/^#\s(.+)$/gm, '<h1>$1</h1>');
            // HR
            html = html.replace(/^---+$/gm, '<hr>');
            // Bold + Italic
            html = html.replace(/\*\*\*(.+?)\*\*\*/g, '<strong><em>$1</em></strong>');
            html = html.replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>');
            html = html.replace(/\*(.+?)\*/g, '<em>$1</em>');
            // Images
            html = html.replace(/!\[([^\]]*)\]\(([^)]+)\)/g, '<img src="$2" alt="$1">');
            // Links
            html = html.replace(/\[([^\]]+)\]\(([^)]+)\)/g, '<a href="$2" target="_blank">$1</a>');
            // Blockquotes
            html = html.replace(/^>\s(.+)$/gm, '<blockquote>$1</blockquote>');
            // Unordered lists
            html = html.replace(/(^[-*]\s.+\n?)+/gm, function(block) {
                const items = block.trim().split('\n').map(l => '<li>' + l.replace(/^[-*]\s/, '') + '</li>').join('');
                return '<ul>' + items + '</ul>';
            });
            // Ordered lists
            html = html.replace(/(^\d+\.\s.+\n?)+/gm, function(block) {
                const items = block.trim().split('\n').map(l => '<li>' + l.replace(/^\d+\.\s/, '') + '</li>').join('');
                return '<ol>' + items + '</ol>';
            });
            // Paragraphs
            html = html.replace(/\n{2,}/g, '</p><p>');
            html = html.replace(/\n/g, '<br>');
            html = '<p>' + html + '</p>';
            // Clean up empty paragraphs around block elements
            html = html.replace(/<p>\s*(<(?:h[1-6]|pre|blockquote|ul|ol|table|hr)[^>]*>)/g, '$1');
            html = html.replace(/(<\/(?:h[1-6]|pre|blockquote|ul|ol|table|hr)>)\s*<\/p>/g, '$1');
            html = html.replace(/<p>\s*<\/p>/g, '');
            return html;
        }
        const update = () => {
            document.getElementById('md-out').innerHTML = renderMd(document.getElementById('md-in').value);
        };
        document.getElementById('md-in').addEventListener('input', update);
        update();
    }
});
