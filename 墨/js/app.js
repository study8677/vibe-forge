const App = {
    tools: [],
    currentView: 'home',
    currentCategory: 'all',
    _toastTimer: null,

    registerTool(tool) {
        this.tools.push(tool);
    },

    init() {
        this.setupNav();
        this.setupSearch();
        this.setupRouter();
        this.handleRoute();
        document.getElementById('tool-count').textContent = this.tools.length + ' TOOLS';
    },

    catNames: {
        encoding: '编解码',
        crypto: '加密解密',
        text: '文本工具',
        image: '图片工具',
        dev: '开发工具'
    },

    setupNav() {
        document.getElementById('nav').addEventListener('click', (e) => {
            const btn = e.target.closest('.nav-btn');
            if (!btn) return;
            this.currentCategory = btn.dataset.cat;
            document.querySelectorAll('.nav-btn').forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
            if (this.currentView !== 'home') window.location.hash = '';
            else this.renderGrid();
        });
    },

    setupSearch() {
        document.getElementById('search').addEventListener('input', (e) => {
            if (this.currentView !== 'home') window.location.hash = '';
            this.renderGrid(e.target.value.trim().toLowerCase());
        });
    },

    setupRouter() {
        window.addEventListener('hashchange', () => this.handleRoute());
    },

    handleRoute() {
        const hash = window.location.hash.slice(1);
        if (hash && hash.startsWith('tool/')) {
            this.showTool(hash.slice(5));
        } else {
            this.showHome();
        }
    },

    showHome() {
        this.currentView = 'home';
        document.getElementById('nav').style.display = '';
        this.renderGrid(document.getElementById('search').value.trim().toLowerCase());
    },

    showTool(toolId) {
        const tool = this.tools.find(t => t.id === toolId);
        if (!tool) return this.showHome();

        this.currentView = 'tool';
        document.getElementById('nav').style.display = 'none';

        const main = document.getElementById('main');
        main.innerHTML =
            '<div class="workspace">' +
                '<div class="workspace-header">' +
                    '<button class="back-btn" id="back-btn">\u2190 返回</button>' +
                    '<h2 class="workspace-title">' + tool.name + '</h2>' +
                    '<span class="workspace-desc">' + this.catNames[tool.category] + ' \u00b7 ' + tool.desc + '</span>' +
                '</div>' +
                '<div class="workspace-body" id="workspace-body">' + tool.render() + '</div>' +
            '</div>';

        document.getElementById('back-btn').addEventListener('click', () => {
            window.location.hash = '';
        });
        if (tool.init) tool.init();
    },

    renderGrid(search) {
        search = search || '';
        const main = document.getElementById('main');
        let list = this.tools;

        if (this.currentCategory !== 'all') {
            list = list.filter(t => t.category === this.currentCategory);
        }
        if (search) {
            list = list.filter(t =>
                t.name.toLowerCase().includes(search) ||
                t.desc.toLowerCase().includes(search) ||
                t.id.includes(search)
            );
        }

        if (!list.length) {
            main.innerHTML =
                '<div class="no-results">' +
                    '<div class="no-results-icon">\u2205</div>' +
                    '<p>没有找到匹配的工具</p>' +
                '</div>';
            return;
        }

        let html = '<div class="grid">';
        list.forEach((t, i) => {
            html +=
                '<div class="card" data-tool="' + t.id + '" style="animation-delay:' + (i * 30) + 'ms">' +
                    '<div class="card-icon">' + t.icon + '</div>' +
                    '<div class="card-name">' + t.name + '</div>' +
                    '<div class="card-desc">' + t.desc + '</div>' +
                    '<div class="card-cat">' + this.catNames[t.category] + '</div>' +
                '</div>';
        });
        html += '</div>';
        main.innerHTML = html;

        main.querySelectorAll('.card').forEach(card => {
            card.addEventListener('click', () => {
                window.location.hash = 'tool/' + card.dataset.tool;
            });
        });
    },

    copyToClipboard(text) {
        if (!text) return;
        navigator.clipboard.writeText(text).then(() => {
            this.showToast('已复制到剪贴板');
        }).catch(() => {
            const ta = document.createElement('textarea');
            ta.value = text;
            ta.style.cssText = 'position:fixed;opacity:0';
            document.body.appendChild(ta);
            ta.select();
            document.execCommand('copy');
            document.body.removeChild(ta);
            this.showToast('已复制到剪贴板');
        });
    },

    showToast(msg) {
        const el = document.getElementById('toast');
        el.textContent = msg;
        el.classList.add('show');
        clearTimeout(this._toastTimer);
        this._toastTimer = setTimeout(() => el.classList.remove('show'), 2000);
    },

    downloadFile(dataUrl, filename) {
        const a = document.createElement('a');
        a.href = dataUrl;
        a.download = filename;
        a.click();
    },

    escapeHtml(str) {
        return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
    }
};
