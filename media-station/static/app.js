// ============================================================
//  Media Station — Frontend Application
//  Lightweight SPA with gallery / timeline / folder views,
//  infinite scroll, lazy loading, and lightbox.
// ============================================================

const API = {
  async get(url)  { const r = await fetch(url); if (!r.ok) throw r; return r.json(); },
  async post(url) { const r = await fetch(url, { method: 'POST' }); return r.json(); },

  media(params)     { return this.get('/api/media?' + new URLSearchParams(params)); },
  mediaById(id)     { return this.get(`/api/media/${id}`); },
  dirs(path)        { return this.get('/api/dirs' + (path ? '?path=' + encodeURIComponent(path) : '')); },
  timeline()        { return this.get('/api/timeline'); },
  stats()           { return this.get('/api/stats'); },
  triggerScan()     { return this.post('/api/scan'); },
  scanStatus()      { return this.get('/api/scan/status'); },

  thumbUrl(id, sz)  { return `/api/thumb/${id}/${sz || 'sm'}`; },
  fileUrl(id)       { return `/api/file/${id}`; },
};

// ============================================================
//  App
// ============================================================
class MediaApp {
  constructor() {
    this.view       = 'gallery';
    this.items      = [];
    this.page       = 1;
    this.perPage    = 120;
    this.hasMore    = true;
    this.loading    = false;
    this.mediaType  = '';
    this.sortBy     = 'modified';
    this.searchQ    = '';
    this.dirFilter  = '';
    this.lbIndex    = -1;

    this._bindDOM();
    this._bindEvents();
    this._refreshStats();
    this._loadMedia();
    this._pollScan();
  }

  // -------- DOM refs --------
  _bindDOM() {
    this.grid       = document.getElementById('gallery-grid');
    this.content    = document.getElementById('content-area');
    this.loadingEl  = document.getElementById('loading-indicator');
    this.emptyEl    = document.getElementById('empty-state');
    this.statsBar   = document.getElementById('stats-bar');
    this.sidebar    = document.getElementById('folder-sidebar');
    this.folderTree = document.getElementById('folder-tree');
    this.lightbox   = document.getElementById('lightbox');
    this.lbMedia    = this.lightbox.querySelector('.lb-media');
    this.lbInfo     = this.lightbox.querySelector('.lb-info-bar');
  }

  // -------- Events --------
  _bindEvents() {
    // Nav
    document.querySelectorAll('.nav-btn').forEach(b =>
      b.addEventListener('click', () => this._switchView(b.dataset.view)));

    // Search (debounced)
    let t;
    document.getElementById('search-input').addEventListener('input', e => {
      clearTimeout(t);
      t = setTimeout(() => { this.searchQ = e.target.value; this._reset(); }, 280);
    });

    // Filters
    document.getElementById('type-filter').addEventListener('change', e => {
      this.mediaType = e.target.value; this._reset();
    });
    document.getElementById('sort-select').addEventListener('change', e => {
      this.sortBy = e.target.value; this._reset();
    });

    // Scan
    document.getElementById('scan-btn').addEventListener('click', () => {
      API.triggerScan();
      document.getElementById('scan-btn').classList.add('scanning');
    });

    // Infinite scroll
    this.content.addEventListener('scroll', () => {
      const { scrollTop, scrollHeight, clientHeight } = this.content;
      if (scrollHeight - scrollTop - clientHeight < 600 && !this.loading && this.hasMore) {
        this._loadMore();
      }
    });

    // Lightbox
    this.lightbox.querySelector('.lb-backdrop').addEventListener('click', () => this._closeLB());
    this.lightbox.querySelector('.lb-close').addEventListener('click',    () => this._closeLB());
    this.lightbox.querySelector('.lb-prev').addEventListener('click',     () => this._navLB(-1));
    this.lightbox.querySelector('.lb-next').addEventListener('click',     () => this._navLB(1));

    document.addEventListener('keydown', e => {
      if (this.lightbox.classList.contains('hidden')) return;
      if (e.key === 'Escape')     this._closeLB();
      if (e.key === 'ArrowLeft')  this._navLB(-1);
      if (e.key === 'ArrowRight') this._navLB(1);
    });
  }

  // -------- View switching --------
  _switchView(view) {
    this.view = view;
    document.querySelectorAll('.nav-btn').forEach(b =>
      b.classList.toggle('active', b.dataset.view === view));

    this.sidebar.classList.toggle('hidden', view !== 'folders');

    if (view === 'folders') {
      this._loadFolders();
    } else {
      this.dirFilter = '';
    }
    this._reset();
  }

  // -------- Data loading --------
  _reset() {
    this.items   = [];
    this.page    = 1;
    this.hasMore = true;
    this.grid.innerHTML = '';
    this.emptyEl.classList.add('hidden');
    this._loadMedia();
  }

  async _loadMedia() {
    if (this.loading) return;
    this.loading = true;
    this.loadingEl.classList.remove('hidden');

    try {
      const params = { page: this.page, per_page: this.perPage, sort: this.sortBy };
      if (this.mediaType) params.type = this.mediaType;
      if (this.searchQ)   params.q    = this.searchQ;
      if (this.dirFilter) params.dir  = this.dirFilter;

      const data = await API.media(params);
      this.hasMore = data.has_more;
      this.items.push(...data.items);

      if (this.items.length === 0 && !this.hasMore) {
        this.emptyEl.classList.remove('hidden');
      }

      if (this.view === 'timeline') {
        this._renderTimeline(data.items);
      } else {
        this._renderGrid(data.items);
      }
    } catch (err) {
      console.error('Load failed:', err);
    } finally {
      this.loading = false;
      this.loadingEl.classList.add('hidden');
    }
  }

  _loadMore() { this.page++; this._loadMedia(); }

  // -------- Grid rendering --------
  _renderGrid(newItems) {
    const frag = document.createDocumentFragment();
    const baseIdx = this.items.length - newItems.length;

    newItems.forEach((item, i) => {
      frag.appendChild(this._createCard(item, baseIdx + i));
    });

    this.grid.appendChild(frag);
  }

  _createCard(item, index) {
    const el = document.createElement('div');
    el.className = 'media-item';

    // Placeholder
    const ph = document.createElement('div');
    ph.className = 'placeholder';
    ph.textContent = item.media_type === 'video' ? '▶' : '◻';
    el.appendChild(ph);

    // Lazy-loaded thumbnail
    if (item.thumb_status === 1) {
      const img = document.createElement('img');
      img.loading = 'lazy';
      img.src = API.thumbUrl(item.id, 'sm');
      img.alt = item.filename;
      img.onload = () => { img.classList.add('loaded'); ph.remove(); };
      img.onerror = () => { img.remove(); };
      el.appendChild(img);
    }

    // Video badge
    if (item.media_type === 'video') {
      const badge = document.createElement('span');
      badge.className = 'badge';
      badge.textContent = '▶ VIDEO';
      el.appendChild(badge);
    }

    // Info overlay
    const info = document.createElement('div');
    info.className = 'info-overlay';
    info.textContent = item.filename;
    el.appendChild(info);

    el.addEventListener('click', () => this._openLB(index));
    return el;
  }

  // -------- Timeline rendering --------
  _renderTimeline(newItems) {
    // Group by YYYY-MM
    const groups = new Map();
    newItems.forEach(item => {
      const d = item.taken_at || item.file_modified_at || '';
      const key = d.substring(0, 7) || 'Unknown';
      if (!groups.has(key)) groups.set(key, []);
      groups.get(key).push(item);
    });

    const frag = document.createDocumentFragment();
    const baseIdx = this.items.length - newItems.length;
    let offset = 0;

    for (const [month, items] of groups) {
      // Header (only if not already present)
      if (!this.grid.querySelector(`[data-month="${month}"]`)) {
        const hdr = document.createElement('div');
        hdr.className = 'timeline-header';
        hdr.dataset.month = month;
        hdr.innerHTML = `${this._fmtMonth(month)}<span class="count">${items.length} items</span>`;
        frag.appendChild(hdr);
      }

      items.forEach(item => {
        frag.appendChild(this._createCard(item, baseIdx + offset));
        offset++;
      });
    }

    this.grid.appendChild(frag);
  }

  _fmtMonth(m) {
    if (m === 'Unknown') return m;
    const [y, mo] = m.split('-');
    const names = ['Jan','Feb','Mar','Apr','May','Jun','Jul','Aug','Sep','Oct','Nov','Dec'];
    return `${names[parseInt(mo,10) - 1] || mo} ${y}`;
  }

  // -------- Lightbox --------
  _openLB(index) {
    this.lbIndex = index;
    this.lightbox.classList.remove('hidden');
    document.body.style.overflow = 'hidden';
    this._renderLB();
  }

  _closeLB() {
    this.lightbox.classList.add('hidden');
    document.body.style.overflow = '';
    // Stop any playing video
    const vid = this.lbMedia.querySelector('video');
    if (vid) vid.pause();
    this.lbMedia.innerHTML = '';
  }

  _navLB(dir) {
    const next = this.lbIndex + dir;
    if (next < 0 || next >= this.items.length) return;
    // Stop current video
    const vid = this.lbMedia.querySelector('video');
    if (vid) vid.pause();
    this.lbIndex = next;
    this._renderLB();

    // Pre-load more items if near end
    if (next >= this.items.length - 10 && this.hasMore && !this.loading) {
      this._loadMore();
    }
  }

  _renderLB() {
    const item = this.items[this.lbIndex];
    if (!item) return;

    this.lbMedia.innerHTML = '';

    if (item.media_type === 'video') {
      const v = document.createElement('video');
      v.src = API.fileUrl(item.id);
      v.controls = true;
      v.autoplay = true;
      v.playsInline = true;
      this.lbMedia.appendChild(v);
    } else {
      const img = document.createElement('img');
      // Use medium thumb first, clicking loads full
      img.src = item.thumb_status === 1 ? API.thumbUrl(item.id, 'md') : API.fileUrl(item.id);
      img.alt = item.filename;
      img.addEventListener('click', () => {
        if (img.src !== API.fileUrl(item.id)) {
          img.src = API.fileUrl(item.id);
        }
      });
      img.title = 'Click for full resolution';
      this.lbMedia.appendChild(img);
    }

    // Info bar
    const dims = item.width && item.height ? `${item.width} x ${item.height}` : '';
    const date = item.taken_at
      ? item.taken_at.replace('T', ' ').substring(0, 19)
      : item.file_modified_at.replace('T', ' ').substring(0, 19);

    this.lbInfo.innerHTML = `
      <strong>${item.filename}</strong>
      <span>${this._fmtSize(item.file_size)}</span>
      ${dims ? `<span>${dims}</span>` : ''}
      <span>${date}</span>
      <span style="margin-left:auto;color:var(--text-3)">${this.lbIndex + 1} / ${this.items.length}</span>
    `;
  }

  // -------- Folders --------
  async _loadFolders(parentPath) {
    try {
      const dirs = await API.dirs(parentPath || '');
      this.folderTree.innerHTML = '';

      // Back button
      if (parentPath) {
        const back = document.createElement('div');
        back.className = 'folder-back';
        back.textContent = '← Back';
        back.addEventListener('click', () => {
          const parent = parentPath.substring(0, parentPath.lastIndexOf('/'));
          this.dirFilter = '';
          this._reset();
          this._loadFolders(parent || undefined);
        });
        this.folderTree.appendChild(back);
      }

      dirs.forEach(dir => {
        const el = document.createElement('div');
        el.className = 'folder-item' + (this.dirFilter === dir.path ? ' active' : '');
        el.innerHTML = `<span class="icon">📁</span>${dir.name}<span class="cnt">${dir.count}</span>`;
        el.addEventListener('click', () => {
          // Show files in this directory
          this.dirFilter = dir.path;
          this._reset();

          // Mark active
          this.folderTree.querySelectorAll('.folder-item').forEach(f => f.classList.remove('active'));
          el.classList.add('active');

          // If it has children, allow drilling down on double-click
          if (dir.has_children) {
            el.addEventListener('dblclick', () => this._loadFolders(dir.path));
          }
        });

        // Single-click to expand children in-place
        if (dir.has_children) {
          el.addEventListener('dblclick', () => this._loadFolders(dir.path));
        }

        this.folderTree.appendChild(el);
      });
    } catch (err) {
      console.error('Failed to load folders:', err);
    }
  }

  // -------- Stats / scan polling --------
  async _refreshStats() {
    try {
      const s = await API.stats();
      this.statsBar.textContent =
        `${s.total_photos.toLocaleString()} photos · ${s.total_videos.toLocaleString()} videos · ` +
        `${this._fmtSize(s.total_size)} · ` +
        `Thumbnails: ${s.thumbs_done.toLocaleString()} / ${(s.total_photos).toLocaleString()}`;
    } catch { /* will retry */ }
  }

  _pollScan() {
    setInterval(async () => {
      try {
        const st = await API.scanStatus();
        const btn = document.getElementById('scan-btn');
        if (st.scanning) {
          btn.classList.add('scanning');
          btn.textContent = 'Scanning...';
          this.statsBar.textContent =
            `Scanning — ${st.files_found.toLocaleString()} found, ` +
            `${st.files_indexed.toLocaleString()} indexed`;
        } else {
          if (btn.classList.contains('scanning')) {
            btn.classList.remove('scanning');
            btn.textContent = 'Scan';
            this._refreshStats();
            // Reload current view if scan just finished
            if (this.items.length === 0) this._reset();
          }
        }
      } catch { /* ignore */ }
    }, 2500);
  }

  // -------- Utilities --------
  _fmtSize(bytes) {
    if (!bytes) return '0 B';
    const units = ['B', 'KB', 'MB', 'GB', 'TB'];
    let i = 0;
    let size = bytes;
    while (size >= 1024 && i < units.length - 1) { size /= 1024; i++; }
    return size.toFixed(i === 0 ? 0 : 1) + ' ' + units[i];
  }
}

// ============================================================
//  Boot
// ============================================================
document.addEventListener('DOMContentLoaded', () => new MediaApp());
