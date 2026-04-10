// ─── MD5 哈希 ───
App.registerTool({
    id: 'md5', name: 'MD5 哈希', desc: 'MD5 消息摘要算法',
    icon: 'MD5', category: 'crypto',
    render() {
        return '<div class="tool-section">' +
            '<label class="tool-label">输入文本</label>' +
            '<textarea class="tool-input" id="md5-in" placeholder="输入需要计算 MD5 的文本..." style="min-height:150px"></textarea>' +
            '</div>' +
            '<div class="tool-section">' +
            '<label class="tool-label">MD5 哈希值</label>' +
            '<div class="tool-row" style="gap:8px;margin:8px 0">' +
            '<input type="text" class="tool-text-input" id="md5-out-32" readonly placeholder="32 位">' +
            '</div>' +
            '<div class="tool-row" style="gap:8px;margin:8px 0">' +
            '<input type="text" class="tool-text-input" id="md5-out-16" readonly placeholder="16 位">' +
            '</div>' +
            '</div>' +
            '<div class="tool-options">' +
            '<label class="tool-option"><input type="checkbox" class="tool-checkbox" id="md5-upper"> 大写输出</label>' +
            '</div>' +
            '<div class="btn-group">' +
            '<button class="btn btn-primary" id="md5-calc">计算 MD5</button>' +
            '<button class="btn" id="md5-copy32">复制 32 位</button>' +
            '<button class="btn" id="md5-copy16">复制 16 位</button>' +
            '<button class="btn" id="md5-clear">清空</button>' +
            '</div>';
    },
    init() {
        const calc = () => {
            const v = document.getElementById('md5-in').value;
            const up = document.getElementById('md5-upper').checked;
            let hash32 = CryptoJS.MD5(v).toString();
            if (up) hash32 = hash32.toUpperCase();
            document.getElementById('md5-out-32').value = hash32;
            document.getElementById('md5-out-16').value = hash32.substring(8, 24);
        };
        document.getElementById('md5-calc').onclick = calc;
        document.getElementById('md5-copy32').onclick = () => App.copyToClipboard(document.getElementById('md5-out-32').value);
        document.getElementById('md5-copy16').onclick = () => App.copyToClipboard(document.getElementById('md5-out-16').value);
        document.getElementById('md5-clear').onclick = () => {
            document.getElementById('md5-in').value = '';
            document.getElementById('md5-out-32').value = '';
            document.getElementById('md5-out-16').value = '';
        };
    }
});

// ─── SHA 哈希 ───
App.registerTool({
    id: 'sha', name: 'SHA 哈希', desc: 'SHA-1 / SHA-256 / SHA-512',
    icon: 'SHA', category: 'crypto',
    render() {
        return '<div class="tool-section">' +
            '<label class="tool-label">输入文本</label>' +
            '<textarea class="tool-input" id="sha-in" placeholder="输入需要计算哈希的文本..." style="min-height:150px"></textarea>' +
            '</div>' +
            '<div class="tool-options">' +
            '<label class="tool-option">算法 <select class="tool-select" id="sha-algo">' +
            '<option value="SHA1">SHA-1</option>' +
            '<option value="SHA256" selected>SHA-256</option>' +
            '<option value="SHA384">SHA-384</option>' +
            '<option value="SHA512">SHA-512</option>' +
            '</select></label>' +
            '<label class="tool-option"><input type="checkbox" class="tool-checkbox" id="sha-upper"> 大写</label>' +
            '</div>' +
            '<div class="tool-section">' +
            '<label class="tool-label">哈希结果</label>' +
            '<textarea class="tool-output" id="sha-out" readonly style="min-height:80px"></textarea>' +
            '</div>' +
            '<div class="btn-group">' +
            '<button class="btn btn-primary" id="sha-calc">计算</button>' +
            '<button class="btn" id="sha-copy">复制结果</button>' +
            '<button class="btn" id="sha-clear">清空</button>' +
            '</div>';
    },
    init() {
        document.getElementById('sha-calc').onclick = () => {
            const v = document.getElementById('sha-in').value;
            const algo = document.getElementById('sha-algo').value;
            const up = document.getElementById('sha-upper').checked;
            let hash = CryptoJS[algo](v).toString();
            if (up) hash = hash.toUpperCase();
            document.getElementById('sha-out').value = hash;
        };
        document.getElementById('sha-copy').onclick = () => App.copyToClipboard(document.getElementById('sha-out').value);
        document.getElementById('sha-clear').onclick = () => {
            document.getElementById('sha-in').value = '';
            document.getElementById('sha-out').value = '';
        };
    }
});

// ─── HMAC 生成 ───
App.registerTool({
    id: 'hmac', name: 'HMAC', desc: 'HMAC 消息认证码生成',
    icon: 'MAC', category: 'crypto',
    render() {
        return '<div class="tool-section">' +
            '<label class="tool-label">消息</label>' +
            '<textarea class="tool-input" id="hmac-msg" placeholder="输入消息..." style="min-height:120px"></textarea>' +
            '</div>' +
            '<div class="tool-section">' +
            '<label class="tool-label">密钥</label>' +
            '<input type="text" class="tool-text-input" id="hmac-key" placeholder="输入密钥...">' +
            '</div>' +
            '<div class="tool-options">' +
            '<label class="tool-option">算法 <select class="tool-select" id="hmac-algo">' +
            '<option value="HmacMD5">HMAC-MD5</option>' +
            '<option value="HmacSHA1">HMAC-SHA1</option>' +
            '<option value="HmacSHA256" selected>HMAC-SHA256</option>' +
            '<option value="HmacSHA512">HMAC-SHA512</option>' +
            '</select></label>' +
            '<label class="tool-option"><input type="checkbox" class="tool-checkbox" id="hmac-upper"> 大写</label>' +
            '</div>' +
            '<div class="tool-section">' +
            '<label class="tool-label">HMAC 结果</label>' +
            '<textarea class="tool-output" id="hmac-out" readonly style="min-height:80px"></textarea>' +
            '</div>' +
            '<div class="btn-group">' +
            '<button class="btn btn-primary" id="hmac-calc">计算</button>' +
            '<button class="btn" id="hmac-copy">复制结果</button>' +
            '<button class="btn" id="hmac-clear">清空</button>' +
            '</div>';
    },
    init() {
        document.getElementById('hmac-calc').onclick = () => {
            const msg = document.getElementById('hmac-msg').value;
            const key = document.getElementById('hmac-key').value;
            const algo = document.getElementById('hmac-algo').value;
            const up = document.getElementById('hmac-upper').checked;
            let hash = CryptoJS[algo](msg, key).toString();
            if (up) hash = hash.toUpperCase();
            document.getElementById('hmac-out').value = hash;
        };
        document.getElementById('hmac-copy').onclick = () => App.copyToClipboard(document.getElementById('hmac-out').value);
        document.getElementById('hmac-clear').onclick = () => {
            document.getElementById('hmac-msg').value = '';
            document.getElementById('hmac-key').value = '';
            document.getElementById('hmac-out').value = '';
        };
    }
});

// ─── AES 加解密 ───
App.registerTool({
    id: 'aes', name: 'AES 加解密', desc: 'AES 对称加密与解密',
    icon: 'AES', category: 'crypto',
    render() {
        return '<div class="tool-row"><div class="tool-col">' +
            '<label class="tool-label">输入</label>' +
            '<textarea class="tool-input" id="aes-in" placeholder="输入明文或密文..."></textarea>' +
            '</div><div class="tool-col">' +
            '<label class="tool-label">输出</label>' +
            '<textarea class="tool-output" id="aes-out" readonly></textarea>' +
            '</div></div>' +
            '<div class="tool-section">' +
            '<label class="tool-label">密钥</label>' +
            '<input type="text" class="tool-text-input" id="aes-key" placeholder="输入密钥 (任意长度)">' +
            '</div>' +
            '<div class="tool-options">' +
            '<label class="tool-option">模式 <select class="tool-select" id="aes-mode">' +
            '<option value="CBC">CBC</option>' +
            '<option value="ECB">ECB</option>' +
            '<option value="CFB">CFB</option>' +
            '</select></label>' +
            '<label class="tool-option">填充 <select class="tool-select" id="aes-pad">' +
            '<option value="Pkcs7">PKCS7</option>' +
            '<option value="ZeroPadding">ZeroPadding</option>' +
            '<option value="NoPadding">NoPadding</option>' +
            '</select></label>' +
            '</div>' +
            '<div class="btn-group">' +
            '<button class="btn btn-primary" id="aes-enc">加密</button>' +
            '<button class="btn btn-primary" id="aes-dec">解密</button>' +
            '<button class="btn" id="aes-copy">复制结果</button>' +
            '<button class="btn" id="aes-clear">清空</button>' +
            '</div>';
    },
    init() {
        const getOpts = () => {
            const mode = document.getElementById('aes-mode').value;
            const pad = document.getElementById('aes-pad').value;
            const key = CryptoJS.enc.Utf8.parse(document.getElementById('aes-key').value);
            const iv = CryptoJS.enc.Utf8.parse('0000000000000000');
            return { mode: CryptoJS.mode[mode], padding: CryptoJS.pad[pad], iv: iv, key: key };
        };
        document.getElementById('aes-enc').onclick = () => {
            try {
                const opts = getOpts();
                const encrypted = CryptoJS.AES.encrypt($i('aes-in').value, opts.key, opts);
                $i('aes-out').value = encrypted.toString();
            } catch (e) { $i('aes-out').value = '加密错误: ' + e.message; }
        };
        document.getElementById('aes-dec').onclick = () => {
            try {
                const opts = getOpts();
                const decrypted = CryptoJS.AES.decrypt($i('aes-in').value.trim(), opts.key, opts);
                const text = decrypted.toString(CryptoJS.enc.Utf8);
                $i('aes-out').value = text || '解密结果为空 (密钥可能不正确)';
            } catch (e) { $i('aes-out').value = '解密错误: ' + e.message; }
        };
        document.getElementById('aes-copy').onclick = () => App.copyToClipboard($i('aes-out').value);
        document.getElementById('aes-clear').onclick = () => {
            $i('aes-in').value = ''; $i('aes-out').value = ''; $i('aes-key').value = '';
        };
        function $i(id) { return document.getElementById(id); }
    }
});

// ─── RSA 密钥生成 ───
App.registerTool({
    id: 'rsa', name: 'RSA 密钥', desc: 'RSA 密钥对生成',
    icon: 'RSA', category: 'crypto',
    render() {
        return '<div class="tool-options">' +
            '<label class="tool-option">密钥长度 <select class="tool-select" id="rsa-bits">' +
            '<option value="1024">1024 位</option>' +
            '<option value="2048" selected>2048 位</option>' +
            '<option value="4096">4096 位</option>' +
            '</select></label>' +
            '</div>' +
            '<div class="btn-group" style="margin-bottom:20px">' +
            '<button class="btn btn-primary" id="rsa-gen">生成密钥对</button>' +
            '</div>' +
            '<div class="tool-row"><div class="tool-col">' +
            '<label class="tool-label">公钥 (SPKI PEM)</label>' +
            '<textarea class="tool-output" id="rsa-pub" readonly style="min-height:200px"></textarea>' +
            '<button class="btn btn-sm" id="rsa-copy-pub" style="margin-top:8px">复制公钥</button>' +
            '</div><div class="tool-col">' +
            '<label class="tool-label">私钥 (PKCS8 PEM)</label>' +
            '<textarea class="tool-output" id="rsa-priv" readonly style="min-height:200px"></textarea>' +
            '<button class="btn btn-sm" id="rsa-copy-priv" style="margin-top:8px">复制私钥</button>' +
            '</div></div>';
    },
    init() {
        function arrayBufToBase64(buf) {
            let bin = '';
            const bytes = new Uint8Array(buf);
            bytes.forEach(b => bin += String.fromCharCode(b));
            return btoa(bin);
        }
        function formatPem(b64, type) {
            const lines = b64.match(/.{1,64}/g) || [];
            return '-----BEGIN ' + type + '-----\n' + lines.join('\n') + '\n-----END ' + type + '-----';
        }
        document.getElementById('rsa-gen').onclick = async () => {
            const bits = parseInt(document.getElementById('rsa-bits').value);
            document.getElementById('rsa-pub').value = '正在生成...';
            document.getElementById('rsa-priv').value = '正在生成...';
            try {
                const keyPair = await crypto.subtle.generateKey(
                    { name: 'RSA-OAEP', modulusLength: bits, publicExponent: new Uint8Array([1, 0, 1]), hash: 'SHA-256' },
                    true, ['encrypt', 'decrypt']
                );
                const pubBuf = await crypto.subtle.exportKey('spki', keyPair.publicKey);
                const privBuf = await crypto.subtle.exportKey('pkcs8', keyPair.privateKey);
                document.getElementById('rsa-pub').value = formatPem(arrayBufToBase64(pubBuf), 'PUBLIC KEY');
                document.getElementById('rsa-priv').value = formatPem(arrayBufToBase64(privBuf), 'PRIVATE KEY');
            } catch (e) {
                document.getElementById('rsa-pub').value = '生成失败: ' + e.message;
                document.getElementById('rsa-priv').value = '';
            }
        };
        document.getElementById('rsa-copy-pub').onclick = () => App.copyToClipboard(document.getElementById('rsa-pub').value);
        document.getElementById('rsa-copy-priv').onclick = () => App.copyToClipboard(document.getElementById('rsa-priv').value);
    }
});

// ─── JWT 解析 ───
App.registerTool({
    id: 'jwt', name: 'JWT 解析', desc: 'JSON Web Token 解析',
    icon: 'JWT', category: 'crypto',
    render() {
        return '<div class="tool-section">' +
            '<label class="tool-label">JWT Token</label>' +
            '<textarea class="tool-input" id="jwt-in" placeholder="粘贴 JWT Token..." style="min-height:100px"></textarea>' +
            '</div>' +
            '<div class="btn-group" style="margin-bottom:20px">' +
            '<button class="btn btn-primary" id="jwt-dec">解析</button>' +
            '<button class="btn" id="jwt-clear">清空</button>' +
            '</div>' +
            '<div class="tool-row"><div class="tool-col">' +
            '<label class="tool-label">Header</label>' +
            '<textarea class="tool-output" id="jwt-header" readonly style="min-height:120px"></textarea>' +
            '</div><div class="tool-col">' +
            '<label class="tool-label">Payload</label>' +
            '<textarea class="tool-output" id="jwt-payload" readonly style="min-height:120px"></textarea>' +
            '</div></div>' +
            '<div class="tool-section">' +
            '<label class="tool-label">Signature (Base64URL)</label>' +
            '<input type="text" class="tool-text-input" id="jwt-sig" readonly>' +
            '</div>' +
            '<div class="tool-section" id="jwt-info"></div>';
    },
    init() {
        function b64urlDecode(str) {
            str = str.replace(/-/g, '+').replace(/_/g, '/');
            while (str.length % 4) str += '=';
            return decodeURIComponent(escape(atob(str)));
        }
        document.getElementById('jwt-dec').onclick = () => {
            const token = document.getElementById('jwt-in').value.trim();
            const parts = token.split('.');
            if (parts.length !== 3) {
                document.getElementById('jwt-header').value = '错误: JWT 应包含 3 个部分 (以 . 分隔)';
                return;
            }
            try {
                const header = JSON.parse(b64urlDecode(parts[0]));
                const payload = JSON.parse(b64urlDecode(parts[1]));
                document.getElementById('jwt-header').value = JSON.stringify(header, null, 2);
                document.getElementById('jwt-payload').value = JSON.stringify(payload, null, 2);
                document.getElementById('jwt-sig').value = parts[2];

                let info = '';
                if (payload.exp) {
                    const expDate = new Date(payload.exp * 1000);
                    const expired = expDate < new Date();
                    info += '<p class="img-info">过期时间: ' + expDate.toLocaleString() + (expired ? ' <b style="color:#a55">(已过期)</b>' : ' <b style="color:#5a5">(有效)</b>') + '</p>';
                }
                if (payload.iat) info += '<p class="img-info">签发时间: ' + new Date(payload.iat * 1000).toLocaleString() + '</p>';
                if (payload.iss) info += '<p class="img-info">签发者: ' + App.escapeHtml(payload.iss) + '</p>';
                if (payload.sub) info += '<p class="img-info">主题: ' + App.escapeHtml(payload.sub) + '</p>';
                document.getElementById('jwt-info').innerHTML = info;
            } catch (e) {
                document.getElementById('jwt-header').value = '解析错误: ' + e.message;
            }
        };
        document.getElementById('jwt-clear').onclick = () => {
            ['jwt-in', 'jwt-header', 'jwt-payload'].forEach(id => document.getElementById(id).value = '');
            document.getElementById('jwt-sig').value = '';
            document.getElementById('jwt-info').innerHTML = '';
        };
    }
});
