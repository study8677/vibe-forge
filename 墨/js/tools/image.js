// ─── 图片压缩 ───
App.registerTool({
    id: 'img-compress', name: '图片压缩', desc: '调整图片质量与尺寸',
    icon: 'ZIP', category: 'image',
    render() {
        return '<div class="tool-section">' +
            '<div class="drop-zone" id="ic-drop">' +
            '<input type="file" accept="image/*" id="ic-file">' +
            '<div class="drop-zone-text">点击或拖拽上传图片</div>' +
            '<div class="drop-zone-hint">支持 JPG / PNG / WebP</div>' +
            '</div></div>' +
            '<div id="ic-preview" style="display:none">' +
            '<div class="img-info" id="ic-info"></div>' +
            '<div class="tool-options">' +
            '<label class="tool-option">质量 <input type="range" class="tool-range" id="ic-quality" min="10" max="100" value="80"> <span id="ic-qval">80%</span></label>' +
            '<label class="tool-option">缩放 <input type="range" class="tool-range" id="ic-scale" min="10" max="100" value="100"> <span id="ic-sval">100%</span></label>' +
            '<label class="tool-option">格式 <select class="tool-select" id="ic-format">' +
            '<option value="image/jpeg">JPEG</option>' +
            '<option value="image/webp">WebP</option>' +
            '<option value="image/png">PNG</option>' +
            '</select></label>' +
            '</div>' +
            '<div class="btn-group" style="margin-bottom:16px">' +
            '<button class="btn btn-primary" id="ic-compress">压缩</button>' +
            '<button class="btn" id="ic-download">下载</button>' +
            '</div>' +
            '<div class="tool-row"><div class="tool-col">' +
            '<label class="tool-label">原图</label>' +
            '<img class="img-preview" id="ic-orig">' +
            '</div><div class="tool-col">' +
            '<label class="tool-label">压缩后</label>' +
            '<canvas class="img-preview" id="ic-canvas" style="display:none"></canvas>' +
            '<div class="img-info" id="ic-result-info"></div>' +
            '</div></div></div>';
    },
    init() {
        let origImg = null;
        const loadImg = (file) => {
            const reader = new FileReader();
            reader.onload = (e) => {
                origImg = new Image();
                origImg.onload = () => {
                    document.getElementById('ic-preview').style.display = '';
                    document.getElementById('ic-orig').src = e.target.result;
                    document.getElementById('ic-info').textContent =
                        '原始尺寸: ' + origImg.width + ' × ' + origImg.height + ' | 文件大小: ' + (file.size / 1024).toFixed(1) + ' KB';
                };
                origImg.src = e.target.result;
            };
            reader.readAsDataURL(file);
        };
        document.getElementById('ic-file').onchange = (e) => { if (e.target.files[0]) loadImg(e.target.files[0]); };
        document.getElementById('ic-quality').oninput = (e) => { document.getElementById('ic-qval').textContent = e.target.value + '%'; };
        document.getElementById('ic-scale').oninput = (e) => { document.getElementById('ic-sval').textContent = e.target.value + '%'; };
        document.getElementById('ic-compress').onclick = () => {
            if (!origImg) return;
            const canvas = document.getElementById('ic-canvas');
            const scale = document.getElementById('ic-scale').value / 100;
            const quality = document.getElementById('ic-quality').value / 100;
            const format = document.getElementById('ic-format').value;
            canvas.width = Math.round(origImg.width * scale);
            canvas.height = Math.round(origImg.height * scale);
            const ctx = canvas.getContext('2d');
            ctx.drawImage(origImg, 0, 0, canvas.width, canvas.height);
            canvas.style.display = '';
            canvas.toBlob(blob => {
                document.getElementById('ic-result-info').textContent =
                    '压缩后: ' + canvas.width + ' × ' + canvas.height + ' | ' + (blob.size / 1024).toFixed(1) + ' KB';
            }, format, quality);
        };
        document.getElementById('ic-download').onclick = () => {
            const canvas = document.getElementById('ic-canvas');
            if (!canvas.width) return;
            const format = document.getElementById('ic-format').value;
            const quality = document.getElementById('ic-quality').value / 100;
            const ext = format.split('/')[1];
            App.downloadFile(canvas.toDataURL(format, quality), 'compressed.' + ext);
        };
    }
});

// ─── 图片裁剪 ───
App.registerTool({
    id: 'img-crop', name: '图片裁剪', desc: '自定义区域裁剪图片',
    icon: 'CRP', category: 'image',
    render() {
        return '<div class="tool-section">' +
            '<div class="drop-zone" id="crop-drop">' +
            '<input type="file" accept="image/*" id="crop-file">' +
            '<div class="drop-zone-text">点击或拖拽上传图片</div>' +
            '</div></div>' +
            '<div id="crop-panel" style="display:none">' +
            '<div class="img-info" id="crop-info"></div>' +
            '<div class="tool-options">' +
            '<label class="tool-option">X <input type="number" class="tool-text-input" id="crop-x" value="0" min="0" style="width:80px"></label>' +
            '<label class="tool-option">Y <input type="number" class="tool-text-input" id="crop-y" value="0" min="0" style="width:80px"></label>' +
            '<label class="tool-option">宽 <input type="number" class="tool-text-input" id="crop-w" value="200" min="1" style="width:80px"></label>' +
            '<label class="tool-option">高 <input type="number" class="tool-text-input" id="crop-h" value="200" min="1" style="width:80px"></label>' +
            '<label class="tool-option">比例 <select class="tool-select" id="crop-ratio">' +
            '<option value="free">自由</option>' +
            '<option value="1:1">1:1</option>' +
            '<option value="4:3">4:3</option>' +
            '<option value="16:9">16:9</option>' +
            '<option value="3:2">3:2</option>' +
            '</select></label>' +
            '</div>' +
            '<div class="btn-group" style="margin-bottom:16px">' +
            '<button class="btn btn-primary" id="crop-run">裁剪预览</button>' +
            '<button class="btn" id="crop-download">下载</button>' +
            '</div>' +
            '<div class="tool-row"><div class="tool-col">' +
            '<label class="tool-label">原图</label>' +
            '<img class="img-preview" id="crop-orig">' +
            '</div><div class="tool-col">' +
            '<label class="tool-label">裁剪结果</label>' +
            '<canvas class="img-preview" id="crop-canvas" style="display:none"></canvas>' +
            '</div></div></div>';
    },
    init() {
        let origImg = null;
        document.getElementById('crop-file').onchange = (e) => {
            const file = e.target.files[0];
            if (!file) return;
            const reader = new FileReader();
            reader.onload = (ev) => {
                origImg = new Image();
                origImg.onload = () => {
                    document.getElementById('crop-panel').style.display = '';
                    document.getElementById('crop-orig').src = ev.target.result;
                    document.getElementById('crop-info').textContent = '尺寸: ' + origImg.width + ' × ' + origImg.height;
                    document.getElementById('crop-w').value = Math.min(200, origImg.width);
                    document.getElementById('crop-h').value = Math.min(200, origImg.height);
                };
                origImg.src = ev.target.result;
            };
            reader.readAsDataURL(file);
        };
        document.getElementById('crop-ratio').onchange = (e) => {
            if (e.target.value === 'free' || !origImg) return;
            const [rw, rh] = e.target.value.split(':').map(Number);
            const w = parseInt(document.getElementById('crop-w').value);
            document.getElementById('crop-h').value = Math.round(w * rh / rw);
        };
        document.getElementById('crop-run').onclick = () => {
            if (!origImg) return;
            const x = parseInt(document.getElementById('crop-x').value) || 0;
            const y = parseInt(document.getElementById('crop-y').value) || 0;
            const w = parseInt(document.getElementById('crop-w').value) || 100;
            const h = parseInt(document.getElementById('crop-h').value) || 100;
            const canvas = document.getElementById('crop-canvas');
            canvas.width = w;
            canvas.height = h;
            canvas.getContext('2d').drawImage(origImg, x, y, w, h, 0, 0, w, h);
            canvas.style.display = '';
        };
        document.getElementById('crop-download').onclick = () => {
            const canvas = document.getElementById('crop-canvas');
            if (!canvas.width) return;
            App.downloadFile(canvas.toDataURL('image/png'), 'cropped.png');
        };
    }
});

// ─── 图片格式转换 ───
App.registerTool({
    id: 'img-convert', name: '格式转换', desc: '图片格式互转',
    icon: 'CVT', category: 'image',
    render() {
        return '<div class="tool-section">' +
            '<div class="drop-zone">' +
            '<input type="file" accept="image/*" id="cvt-file">' +
            '<div class="drop-zone-text">点击或拖拽上传图片</div>' +
            '</div></div>' +
            '<div id="cvt-panel" style="display:none">' +
            '<div class="img-info" id="cvt-info"></div>' +
            '<img class="img-preview" id="cvt-preview">' +
            '<div class="tool-options" style="margin-top:16px">' +
            '<label class="tool-option">目标格式 <select class="tool-select" id="cvt-format">' +
            '<option value="image/png">PNG</option>' +
            '<option value="image/jpeg">JPEG</option>' +
            '<option value="image/webp">WebP</option>' +
            '<option value="image/bmp">BMP</option>' +
            '</select></label>' +
            '<label class="tool-option">JPEG/WebP 质量 <input type="range" class="tool-range" id="cvt-quality" min="10" max="100" value="92"> <span id="cvt-qval">92%</span></label>' +
            '</div>' +
            '<div class="btn-group">' +
            '<button class="btn btn-primary" id="cvt-run">转换并下载</button>' +
            '</div></div>';
    },
    init() {
        let origImg = null;
        document.getElementById('cvt-file').onchange = (e) => {
            const file = e.target.files[0];
            if (!file) return;
            const reader = new FileReader();
            reader.onload = (ev) => {
                origImg = new Image();
                origImg.onload = () => {
                    document.getElementById('cvt-panel').style.display = '';
                    document.getElementById('cvt-preview').src = ev.target.result;
                    document.getElementById('cvt-info').textContent =
                        '原始格式: ' + file.type + ' | 尺寸: ' + origImg.width + ' × ' + origImg.height + ' | 大小: ' + (file.size / 1024).toFixed(1) + ' KB';
                };
                origImg.src = ev.target.result;
            };
            reader.readAsDataURL(file);
        };
        document.getElementById('cvt-quality').oninput = (e) => { document.getElementById('cvt-qval').textContent = e.target.value + '%'; };
        document.getElementById('cvt-run').onclick = () => {
            if (!origImg) return;
            const canvas = document.createElement('canvas');
            canvas.width = origImg.width;
            canvas.height = origImg.height;
            canvas.getContext('2d').drawImage(origImg, 0, 0);
            const format = document.getElementById('cvt-format').value;
            const quality = document.getElementById('cvt-quality').value / 100;
            const ext = format.split('/')[1];
            App.downloadFile(canvas.toDataURL(format, quality), 'converted.' + ext);
            App.showToast('已转换为 ' + ext.toUpperCase());
        };
    }
});

// ─── 图片转 Base64 ───
App.registerTool({
    id: 'img-base64', name: '图片 Base64', desc: '图片与 Base64 互转',
    icon: 'I64', category: 'image',
    render() {
        return '<div class="tool-section">' +
            '<label class="tool-label">图片 → Base64</label>' +
            '<div class="drop-zone">' +
            '<input type="file" accept="image/*" id="i64-file">' +
            '<div class="drop-zone-text">点击或拖拽上传图片</div>' +
            '</div></div>' +
            '<div class="tool-section">' +
            '<label class="tool-label">Base64 字符串</label>' +
            '<textarea class="tool-input" id="i64-text" placeholder="上传图片获取 Base64，或粘贴 Base64 预览图片..." style="min-height:150px"></textarea>' +
            '</div>' +
            '<div class="btn-group" style="margin-bottom:16px">' +
            '<button class="btn" id="i64-copy">复制 Base64</button>' +
            '<button class="btn btn-primary" id="i64-show">Base64 → 预览</button>' +
            '<button class="btn" id="i64-download">下载图片</button>' +
            '<button class="btn" id="i64-clear">清空</button>' +
            '</div>' +
            '<div class="tool-section">' +
            '<label class="tool-label">预览</label>' +
            '<img class="img-preview" id="i64-preview" style="display:none">' +
            '<div class="img-info" id="i64-info"></div>' +
            '</div>';
    },
    init() {
        document.getElementById('i64-file').onchange = (e) => {
            const file = e.target.files[0];
            if (!file) return;
            const reader = new FileReader();
            reader.onload = (ev) => {
                document.getElementById('i64-text').value = ev.target.result;
                const img = document.getElementById('i64-preview');
                img.src = ev.target.result;
                img.style.display = '';
                document.getElementById('i64-info').textContent =
                    '文件: ' + file.name + ' | 大小: ' + (file.size / 1024).toFixed(1) + ' KB | Base64 长度: ' + ev.target.result.length;
            };
            reader.readAsDataURL(file);
        };
        document.getElementById('i64-show').onclick = () => {
            let val = document.getElementById('i64-text').value.trim();
            if (!val) return;
            if (!val.startsWith('data:')) val = 'data:image/png;base64,' + val;
            const img = document.getElementById('i64-preview');
            img.src = val;
            img.style.display = '';
            img.onload = () => { document.getElementById('i64-info').textContent = '尺寸: ' + img.naturalWidth + ' × ' + img.naturalHeight; };
            img.onerror = () => { document.getElementById('i64-info').textContent = 'Base64 无法解析为图片'; };
        };
        document.getElementById('i64-copy').onclick = () => App.copyToClipboard(document.getElementById('i64-text').value);
        document.getElementById('i64-download').onclick = () => {
            const src = document.getElementById('i64-preview').src;
            if (src) App.downloadFile(src, 'image.png');
        };
        document.getElementById('i64-clear').onclick = () => {
            document.getElementById('i64-text').value = '';
            document.getElementById('i64-preview').style.display = 'none';
            document.getElementById('i64-info').textContent = '';
        };
    }
});

// ─── 二维码生成 ───
App.registerTool({
    id: 'qrcode', name: '二维码', desc: '二维码生成器',
    icon: 'QR', category: 'image',
    render() {
        return '<div class="tool-section">' +
            '<label class="tool-label">内容</label>' +
            '<textarea class="tool-input" id="qr-text" placeholder="输入文本或 URL..." style="min-height:100px"></textarea>' +
            '</div>' +
            '<div class="tool-options">' +
            '<label class="tool-option">纠错级别 <select class="tool-select" id="qr-ec">' +
            '<option value="L">L (7%)</option>' +
            '<option value="M" selected>M (15%)</option>' +
            '<option value="Q">Q (25%)</option>' +
            '<option value="H">H (30%)</option>' +
            '</select></label>' +
            '<label class="tool-option">单元大小 <input type="range" class="tool-range" id="qr-size" min="4" max="16" value="8"> <span id="qr-sval">8px</span></label>' +
            '</div>' +
            '<div class="btn-group" style="margin-bottom:16px">' +
            '<button class="btn btn-primary" id="qr-gen">生成</button>' +
            '<button class="btn" id="qr-download">下载 PNG</button>' +
            '</div>' +
            '<div id="qr-output"></div>';
    },
    init() {
        document.getElementById('qr-size').oninput = (e) => { document.getElementById('qr-sval').textContent = e.target.value + 'px'; };
        document.getElementById('qr-gen').onclick = () => {
            const text = document.getElementById('qr-text').value;
            if (!text) return App.showToast('请输入内容');
            const ec = document.getElementById('qr-ec').value;
            const cellSize = parseInt(document.getElementById('qr-size').value);
            try {
                var qr = qrcode(0, ec);
                qr.addData(text);
                qr.make();
                const count = qr.getModuleCount();
                const margin = 4;
                const size = (count + margin * 2) * cellSize;
                const canvas = document.createElement('canvas');
                canvas.width = size;
                canvas.height = size;
                const ctx = canvas.getContext('2d');
                ctx.fillStyle = '#ffffff';
                ctx.fillRect(0, 0, size, size);
                ctx.fillStyle = '#000000';
                for (let row = 0; row < count; row++) {
                    for (let col = 0; col < count; col++) {
                        if (qr.isDark(row, col)) {
                            ctx.fillRect((col + margin) * cellSize, (row + margin) * cellSize, cellSize, cellSize);
                        }
                    }
                }
                const output = document.getElementById('qr-output');
                output.innerHTML = '';
                output.className = 'qr-output';
                output.appendChild(canvas);
                canvas.style.maxWidth = '100%';
                canvas.id = 'qr-canvas';
            } catch (e) { App.showToast('生成失败: ' + e.message); }
        };
        document.getElementById('qr-download').onclick = () => {
            const canvas = document.getElementById('qr-canvas');
            if (!canvas) return App.showToast('请先生成二维码');
            App.downloadFile(canvas.toDataURL('image/png'), 'qrcode.png');
        };
    }
});
