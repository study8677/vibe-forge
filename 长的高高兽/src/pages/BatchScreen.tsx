import BatchUpload from '../components/BatchReview/BatchUpload';
import RiskDashboard from '../components/BatchReview/RiskDashboard';
import ListingTable from '../components/BatchReview/ListingTable';
import RiskDetailDrawer from '../components/BatchReview/RiskDetailDrawer';

export default function BatchScreen() {
  return (
    <>
      <BatchUpload />
      <RiskDashboard />
      <ListingTable />
      <RiskDetailDrawer />
    </>
  );
}
