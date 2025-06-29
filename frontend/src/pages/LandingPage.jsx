import TopSection from "../components/LandingPage/TopSection";
import UpperSection from "../components/LandingPage/UpperSection";
import LowerSection from "../components/LandingPage/LowerSection";
import BottomSection from "../components/LandingPage/BottomSection";
function LandingPage() {
  return (
    <div className="h-screen overflow-y-scroll snap-y snap-mandatory scroll-smooth">
      <div className="min-h-screen snap-start">
        <TopSection />
      </div>
      <div className="min-h-screen snap-start">
        <UpperSection />
      </div>
      <div className="min-h-screen snap-start">
        <LowerSection />
      </div>
      <div className="min-h-screen snap-start">
        <BottomSection />
      </div>
    </div>
  );
}

export default LandingPage;