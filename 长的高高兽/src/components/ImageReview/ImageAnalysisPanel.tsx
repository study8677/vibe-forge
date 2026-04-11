import { Progress, Tag, Divider, Empty } from 'antd';
import {
  CheckCircleFilled,
  CloseCircleFilled,
} from '@ant-design/icons';
import { useAppStore } from '../../store';
import type { ImageCheckResult } from '../../types';

function ResultRow({ item }: { item: ImageCheckResult }) {
  return (
    <div
      style={{
        display: 'flex',
        alignItems: 'flex-start',
        gap: 10,
        padding: '10px 0',
        borderBottom: '1px solid #f5f5f5',
      }}
    >
      {item.pass ? (
        <CheckCircleFilled style={{ color: '#52c41a', fontSize: 18, marginTop: 2 }} />
      ) : (
        <CloseCircleFilled style={{ color: '#ff4d4f', fontSize: 18, marginTop: 2 }} />
      )}

      <div style={{ flex: 1 }}>
        <div className="flex items-center justify-between">
          <span style={{ fontWeight: 600, fontSize: 14 }}>{item.ruleName}</span>
          <Tag
            color={item.pass ? 'success' : 'error'}
            style={{ fontSize: 12, borderRadius: 4 }}
          >
            {item.pass ? '通过' : '未通过'}
          </Tag>
        </div>
        <div style={{ color: '#595959', fontSize: 13, marginTop: 2 }}>
          {item.message}
        </div>
        {item.suggestion && (
          <div style={{ color: '#1677ff', fontSize: 12, marginTop: 4 }}>
            建议：{item.suggestion}
          </div>
        )}
        <Progress
          percent={item.score}
          size="small"
          strokeColor={item.score >= 80 ? '#52c41a' : item.score >= 50 ? '#faad14' : '#ff4d4f'}
          style={{ marginTop: 4 }}
          format={(p) => `${p}%`}
        />
      </div>
    </div>
  );
}

export default function ImageAnalysisPanel() {
  const currentImage = useAppStore((s) => s.currentImage);

  if (!currentImage) {
    return (
      <div
        style={{
          background: '#fff',
          borderRadius: 12,
          padding: 48,
          height: '100%',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
        }}
      >
        <Empty
          image={Empty.PRESENTED_IMAGE_SIMPLE}
          description="上传图片后在此查看合规检测结果"
        />
      </div>
    );
  }

  const passCount = currentImage.results.filter((r) => r.pass).length;
  const total = currentImage.results.length;

  return (
    <div
      style={{
        background: '#fff',
        borderRadius: 12,
        padding: 24,
        height: '100%',
        overflow: 'auto',
      }}
    >
      {/* 总评 */}
      <div className="flex items-center gap-4 mb-2">
        <Progress
          type="circle"
          percent={currentImage.overallScore}
          size={90}
          strokeColor={
            currentImage.overallPass
              ? '#52c41a'
              : currentImage.overallScore >= 50
                ? '#faad14'
                : '#ff4d4f'
          }
          format={() => (
            <span
              style={{
                fontSize: 22,
                fontWeight: 700,
                color: currentImage.overallPass ? '#52c41a' : '#ff4d4f',
              }}
            >
              {currentImage.overallScore}
            </span>
          )}
        />
        <div>
          <div style={{ fontSize: 18, fontWeight: 700 }}>
            {currentImage.overallPass ? (
              <span style={{ color: '#52c41a' }}>合规通过</span>
            ) : (
              <span style={{ color: '#ff4d4f' }}>存在不合规项</span>
            )}
          </div>
          <div style={{ color: '#8c8c8c', fontSize: 13, marginTop: 4 }}>
            {passCount} / {total} 项检测通过
          </div>
          <div style={{ color: '#8c8c8c', fontSize: 13 }}>
            {currentImage.file.name} &middot; {currentImage.width}×{currentImage.height}px &middot;{' '}
            {(currentImage.fileSize / 1024).toFixed(0)} KB
          </div>
        </div>
      </div>

      <Divider style={{ margin: '16px 0 8px' }} />

      {/* 逐项结果 */}
      <div style={{ fontSize: 14, fontWeight: 600, marginBottom: 4 }}>
        亚马逊主图合规检测 ({total} 项)
      </div>

      {currentImage.results.map((item) => (
        <ResultRow key={item.ruleId} item={item} />
      ))}

      {/* 规则说明 */}
      <Divider style={{ margin: '20px 0 12px' }} />
      <div style={{ fontSize: 12, color: '#bfbfbf', lineHeight: 1.8 }}>
        <div style={{ fontWeight: 600, color: '#8c8c8c', marginBottom: 4 }}>亚马逊主图要求摘要</div>
        <ul style={{ paddingLeft: 16, margin: 0 }}>
          <li>纯白背景 RGB(255,255,255)</li>
          <li>最长边 ≥ 1000px（推荐 ≥ 1600px）</li>
          <li>产品占画面 85% 以上</li>
          <li>无水印、边框、文字、Logo</li>
          <li>文件 ≤ 10MB，格式 JPEG/PNG/TIFF/GIF</li>
        </ul>
      </div>
    </div>
  );
}
