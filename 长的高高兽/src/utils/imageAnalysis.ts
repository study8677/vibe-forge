import type { ImageCheckResult, ImageAnalysis } from '../types';

// ── 辅助 ──────────────────────────────────────────────────

function loadImage(file: File): Promise<HTMLImageElement> {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.onload = () => resolve(img);
    img.onerror = reject;
    img.src = URL.createObjectURL(file);
  });
}

function getImageData(img: HTMLImageElement): ImageData {
  const canvas = document.createElement('canvas');
  canvas.width = img.naturalWidth;
  canvas.height = img.naturalHeight;
  const ctx = canvas.getContext('2d')!;
  ctx.drawImage(img, 0, 0);
  return ctx.getImageData(0, 0, canvas.width, canvas.height);
}

// ── 检测规则 ──────────────────────────────────────────────

/** 纯白背景检测 — 采样边缘像素 */
function checkWhiteBackground(imageData: ImageData): ImageCheckResult {
  const { data, width, height } = imageData;
  let white = 0;
  let total = 0;
  const T = 240; // 近白阈值

  const depth = Math.max(3, Math.min(8, Math.floor(Math.min(width, height) * 0.02)));

  // 上下边
  for (let x = 0; x < width; x += 2) {
    for (let d = 0; d < depth; d++) {
      for (const y of [d, height - 1 - d]) {
        const i = (y * width + x) * 4;
        total++;
        if (data[i] >= T && data[i + 1] >= T && data[i + 2] >= T) white++;
      }
    }
  }
  // 左右边
  for (let y = depth; y < height - depth; y += 2) {
    for (let d = 0; d < depth; d++) {
      for (const x of [d, width - 1 - d]) {
        const i = (y * width + x) * 4;
        total++;
        if (data[i] >= T && data[i + 1] >= T && data[i + 2] >= T) white++;
      }
    }
  }

  const ratio = total > 0 ? white / total : 0;
  const score = Math.round(ratio * 100);
  const pass = ratio >= 0.85;

  return {
    ruleId: 'bg_white',
    ruleName: '纯白底检测',
    pass,
    score,
    message: pass
      ? `背景白度 ${score}%，符合纯白底要求`
      : `背景白度 ${score}%，亚马逊主图要求 RGB(255,255,255) 纯白`,
    suggestion: pass ? '' : '使用抠图工具将背景替换为纯白 (#FFFFFF)',
  };
}

/** 分辨率检测 */
function checkResolution(w: number, h: number): ImageCheckResult {
  const longest = Math.max(w, h);
  const shortest = Math.min(w, h);

  let pass = true;
  let msg: string;
  if (longest < 1000) {
    pass = false;
    msg = `最长边 ${longest}px < 1000px，无法启用缩放功能`;
  } else if (longest > 10000) {
    pass = false;
    msg = `最长边 ${longest}px > 10000px，超出上限`;
  } else if (shortest < 500) {
    pass = false;
    msg = `最短边 ${shortest}px < 500px，清晰度不足`;
  } else {
    msg = `${w}×${h}px，符合要求`;
  }

  return {
    ruleId: 'resolution',
    ruleName: '分辨率检测',
    pass,
    score: pass ? 100 : longest >= 500 ? 60 : 20,
    message: msg,
    suggestion: pass ? '' : '建议最长边 ≥ 1600px 以获最佳缩放体验',
  };
}

/** 宽高比检测 */
function checkAspectRatio(w: number, h: number): ImageCheckResult {
  const r = w / h;
  const square = Math.abs(r - 1) < 0.05;
  const ok = r >= 0.5 && r <= 2;

  return {
    ruleId: 'aspect_ratio',
    ruleName: '宽高比检测',
    pass: ok,
    score: square ? 100 : ok ? 70 : 30,
    message: square
      ? '1:1 正方形，最佳比例'
      : ok
        ? `${r.toFixed(2)}:1，可接受，建议 1:1`
        : `${r.toFixed(2)}:1，超出可接受范围`,
    suggestion: ok ? '' : '裁剪为 1:1 正方形',
  };
}

/** 文件大小检测 */
function checkFileSize(bytes: number): ImageCheckResult {
  const mb = bytes / 1048576;
  const pass = mb <= 10;
  return {
    ruleId: 'file_size',
    ruleName: '文件大小',
    pass,
    score: mb <= 5 ? 100 : pass ? 70 : 20,
    message: pass ? `${mb.toFixed(2)} MB，符合要求` : `${mb.toFixed(2)} MB，超过 10 MB 限制`,
    suggestion: pass ? '' : '压缩至 10 MB 以内',
  };
}

/** 格式检测 */
function checkFormat(name: string): ImageCheckResult {
  const ext = name.split('.').pop()?.toLowerCase() ?? '';
  const ok = ['jpg', 'jpeg', 'png', 'tif', 'tiff', 'gif', 'bmp', 'webp'].includes(ext);
  return {
    ruleId: 'format',
    ruleName: '格式检测',
    pass: ok,
    score: ok ? 100 : 0,
    message: ok ? `.${ext} 格式，符合要求` : `.${ext} 不受支持`,
    suggestion: ok ? '' : '转换为 JPEG 或 PNG',
  };
}

/** 产品占比检测 */
function checkProductFill(imageData: ImageData): ImageCheckResult {
  const { data, width, height } = imageData;
  const total = width * height;
  let nonWhite = 0;
  const T = 240;

  // 每 4 个像素采样一次
  for (let i = 0; i < data.length; i += 16) {
    if (data[i] < T || data[i + 1] < T || data[i + 2] < T) nonWhite++;
  }
  nonWhite *= 4;

  const fill = nonWhite / total;
  const pct = Math.round(fill * 100);
  const tooSmall = fill < 0.15;
  const good = fill >= 0.3 && fill <= 0.95;

  return {
    ruleId: 'product_fill',
    ruleName: '产品占比',
    pass: !tooSmall,
    score: good ? 100 : !tooSmall ? 70 : 30,
    message: tooSmall
      ? `产品占比 ~${pct}%，过小`
      : good
        ? `产品占比 ~${pct}%，合适`
        : `产品占比 ~${pct}%，${fill > 0.95 ? '贴近边缘' : '偏小'}`,
    suggestion: tooSmall ? '放大产品使其占据画面主体' : '',
  };
}

/** 色彩有效性 */
function checkColorDiversity(imageData: ImageData): ImageCheckResult {
  const { data } = imageData;
  const set = new Set<string>();
  const step = Math.max(4, Math.floor(data.length / 4000)) * 4;

  for (let i = 0; i < data.length && set.size < 600; i += step) {
    set.add(`${data[i] >> 4},${data[i + 1] >> 4},${data[i + 2] >> 4}`);
  }

  const n = set.size;
  const pass = n > 10;

  return {
    ruleId: 'color_diversity',
    ruleName: '图片有效性',
    pass,
    score: pass ? 100 : 20,
    message: pass ? `${n} 种色域，内容正常` : `仅 ${n} 种色域，可能空白或损坏`,
    suggestion: pass ? '' : '检查图片是否正确上传',
  };
}

/** 边框检测 */
function checkBorders(imageData: ImageData): ImageCheckResult {
  const { data, width, height } = imageData;
  const samples: number[][] = [];
  const step = Math.max(1, Math.floor(width / 80));

  for (let x = 0; x < width; x += step) {
    const ti = x * 4;
    samples.push([data[ti], data[ti + 1], data[ti + 2]]);
    const bi = ((height - 1) * width + x) * 4;
    samples.push([data[bi], data[bi + 1], data[bi + 2]]);
  }

  const nonWhite = samples.filter(
    ([r, g, b]) => r < 230 || g < 230 || b < 230
  );

  let hasFrame = false;
  if (nonWhite.length > samples.length * 0.6 && nonWhite.length > 10) {
    const avg = nonWhite
      .reduce((a, c) => [a[0] + c[0], a[1] + c[1], a[2] + c[2]], [0, 0, 0])
      .map((v) => v / nonWhite.length);
    const uniform = nonWhite.filter(
      ([r, g, b]) =>
        Math.abs(r - avg[0]) < 30 &&
        Math.abs(g - avg[1]) < 30 &&
        Math.abs(b - avg[2]) < 30
    );
    hasFrame = uniform.length > nonWhite.length * 0.7;
  }

  return {
    ruleId: 'borders',
    ruleName: '边框检测',
    pass: !hasFrame,
    score: hasFrame ? 20 : 100,
    message: hasFrame ? '检测到图片可能包含边框/相框' : '未检测到边框',
    suggestion: hasFrame ? '亚马逊主图禁止边框、水印及装饰元素' : '',
  };
}

// ── 主入口 ────────────────────────────────────────────────

export async function analyzeImage(file: File): Promise<ImageAnalysis> {
  const img = await loadImage(file);
  const imgData = getImageData(img);
  const url = URL.createObjectURL(file);

  const results: ImageCheckResult[] = [
    checkFormat(file.name),
    checkFileSize(file.size),
    checkResolution(img.naturalWidth, img.naturalHeight),
    checkAspectRatio(img.naturalWidth, img.naturalHeight),
    checkWhiteBackground(imgData),
    checkProductFill(imgData),
    checkColorDiversity(imgData),
    checkBorders(imgData),
  ];

  const passCount = results.filter((r) => r.pass).length;

  return {
    file,
    url,
    width: img.naturalWidth,
    height: img.naturalHeight,
    fileSize: file.size,
    format: file.name.split('.').pop()?.toUpperCase() ?? 'UNKNOWN',
    results,
    overallPass: results.every((r) => r.pass),
    overallScore: Math.round((passCount / results.length) * 100),
    analyzedAt: new Date(),
  };
}
