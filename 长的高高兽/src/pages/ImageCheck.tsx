import { Col, Row } from 'antd';
import ImageDropZone from '../components/ImageReview/ImageDropZone';
import ImageAnalysisPanel from '../components/ImageReview/ImageAnalysisPanel';

export default function ImageCheck() {
  return (
    <Row gutter={20} style={{ height: 'calc(100vh - 100px)' }}>
      <Col xs={24} lg={10} style={{ display: 'flex', flexDirection: 'column' }}>
        <ImageDropZone />
      </Col>
      <Col xs={24} lg={14}>
        <ImageAnalysisPanel />
      </Col>
    </Row>
  );
}
