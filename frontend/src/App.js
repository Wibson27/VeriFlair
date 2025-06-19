import TopSection from "./components/TopSection";
import UpperSection from "./components/UpperSection";
import LowerSection from "./components/LowerSection";
import BottomSection from "./components/BottomSection";

// function App() {
//   return (
//     <div className="text-white font-sans">
//       <TopSection/>
//       <UpperSection/>
//       <LowerSection/>
//       <BottomSection/>
//     </div>
//   );
// }

function App() {
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

export default App;
