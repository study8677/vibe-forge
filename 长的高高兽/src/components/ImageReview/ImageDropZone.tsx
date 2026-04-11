import { useState, useCallback } from 'react';
import { Upload, message, Spin, Image, List, Button, Empty } from 'antd';
import {
  PictureOutlined,
  DeleteOutlined,
  CheckCircleFilled,
  CloseCircleFilled,
} from '@ant-design/icons';
import { useAppStore } from '../../store';
import { analyzeImage } from '../../utils/imageAnalysis';
import type { ImageAnalysis } from '../../types';

export default function ImageDropZone() {
  const { imageAnalyses, currentImage, addImageAnalysis, setCurrentImage, removeImageAnalysis, clearImages } =
    useAppStore();
  const [loading, setLoading] = useState(false);

  const handleFiles = useCallback(
    async (files: File[]) => {
      setLoading(true);
      try {
        for (const file of files) {
          if (!file.type.startsWith('image/')) {
            message.warning(`${file.name} 不是图片文件`);
            continue;
          }
          const result = await analyzeImage(file);
          addImageAnalysis(result);
        }
        message.success(`已分析 ${files.length} 张图片`);
      } catch {
        message.error('图片分析失败');
      } finally {
        setLoading(false);
      }
    },
    [addImageAnalysis]
  );

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100%', gap: 16 }}>
      {/* 上传区 */}
      <Spin spinning={loading} tip="正在分析图片...">
        <Upload.Dragger
          accept="image/*"
          multiple
          showUploadList={false}
          beforeUpload={(_, fileList) => {
            handleFiles(fileList as unknown as File[]);
            return false;
          }}
          style={{
            padding: '32px 16px',
            borderRadius: 12,
            background: currentImage ? '#fafafa' : '#f6f9ff',
            borderColor: '#d9d9d9',
          }}
        >
          {currentImage ? (
            <div style={{ position: 'relative' }}>
              <Image
                src={currentImage.url}
                preview={false}
                style={{
                  maxHeight: 320,
                  objectFit: 'contain',
                  borderRadius: 8,
                }}
              />
              <div
                style={{
                  position: 'absolute',
                  top: 8,
                  right: 8,
                  background: currentImage.overallPass ? '#52c41a' : '#ff4d4f',
                  color: '#fff',
                  borderRadius: 20,
                  padding: '2px 12px',
                  fontSize: 13,
                  fontWeight: 600,
                }}
              >
                {currentImage.overallPass ? 'PASS' : 'FAIL'}
              </div>
              <div style={{ marginTop: 12, color: '#8c8c8c', fontSize: 13 }}>
                {currentImage.width}×{currentImage.height}px &middot;{' '}
                {(currentImage.fileSize / 1024).toFixed(0)} KB &middot; {currentImage.format}
              </div>
            </div>
          ) : (
            <>
              <PictureOutlined style={{ fontSize: 48, color: '#bfbfbf' }} />
              <p style={{ fontSize: 15, color: '#595959', margin: '12px 0 4px' }}>
                拖拽图片到此处，或点击上传
              </p>
              <p style={{ fontSize: 13, color: '#bfbfbf', margin: 0 }}>
                支持 JPG / PNG / TIFF / GIF / BMP / WebP，可批量上传
              </p>
            </>
          )}
        </Upload.Dragger>
      </Spin>

      {/* 历史列表 */}
      {imageAnalyses.length > 0 && (
        <div
          style={{
            background: '#fff',
            borderRadius: 12,
            padding: 16,
            flex: 1,
            overflow: 'auto',
          }}
        >
          <div className="flex items-center justify-between mb-2">
            <span style={{ fontWeight: 600, fontSize: 14 }}>
              检测记录 ({imageAnalyses.length})
            </span>
            <Button type="link" size="small" danger onClick={clearImages}>
              清空
            </Button>
          </div>

          <List
            size="small"
            dataSource={imageAnalyses}
            renderItem={(item: ImageAnalysis, idx: number) => (
              <List.Item
                style={{
                  cursor: 'pointer',
                  padding: '8px 6px',
                  borderRadius: 8,
                  background: currentImage === item ? '#f0f5ff' : 'transparent',
                }}
                onClick={() => setCurrentImage(item)}
                actions={[
                  <Button
                    key="del"
                    type="text"
                    size="small"
                    danger
                    icon={<DeleteOutlined />}
                    onClick={(e) => {
                      e.stopPropagation();
                      removeImageAnalysis(idx);
                    }}
                  />,
                ]}
              >
                <List.Item.Meta
                  avatar={
                    <img
                      src={item.url}
                      alt=""
                      style={{
                        width: 40,
                        height: 40,
                        objectFit: 'cover',
                        borderRadius: 6,
                        border: '1px solid #f0f0f0',
                      }}
                    />
                  }
                  title={
                    <span style={{ fontSize: 13 }}>
                      {item.overallPass ? (
                        <CheckCircleFilled style={{ color: '#52c41a', marginRight: 6 }} />
                      ) : (
                        <CloseCircleFilled style={{ color: '#ff4d4f', marginRight: 6 }} />
                      )}
                      {item.file.name}
                    </span>
                  }
                  description={
                    <span style={{ fontSize: 12, color: '#8c8c8c' }}>
                      {item.width}×{item.height} &middot; 得分 {item.overallScore}%
                    </span>
                  }
                />
              </List.Item>
            )}
          />
        </div>
      )}

      {imageAnalyses.length === 0 && (
        <div
          style={{
            background: '#fff',
            borderRadius: 12,
            padding: 32,
            flex: 1,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
          }}
        >
          <Empty
            image={Empty.PRESENTED_IMAGE_SIMPLE}
            description="上传产品图片，即时获取亚马逊合规检测报告"
          />
        </div>
      )}
    </div>
  );
}
