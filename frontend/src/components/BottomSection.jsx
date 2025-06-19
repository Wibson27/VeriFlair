import blueBg from '../assets/image/bgbottom.png';   // ukuran: 1440x1204
import blackBg from '../assets/image/bgblack.png'; // ukuran: 1440x348

export default function BottomSection() {
  return (
    <div className="w-full">
      {/* Gambar biru sebagai latar atas */}
      <section
        className="relative w-full bg-no-repeat bg-top bg-[length:100%_856px] text-white"
        style={{
          backgroundImage: `url(${blueBg})`,
          minHeight: '856px',
        }}
      >

        <section
            className="flex items-center justify-center text-center px-6"
            style={{ minHeight: '856px' }}
            >
        <div>
          <h2 className="font-sfpro font-semibold text-5xl md:text-5xl mb-4">
            Ready to Build Your <br /> Web3 Developer Identity?
          </h2>
          <p className="font-sfpro font-normal text-gray-200 max-w-xl mb-6">
            Join the future of decentralized professional reputation. Start earning verified credentials today.
          </p>
          <button className="relative inline-block px-6 py-3 font-sfpro font-normal text-white rounded-full group overflow-hidden transition-all duration-300">
                {/* Gradient border layer */}
                <span className="absolute inset-0 rounded-full bg-gradient-to-r from-purple-500 via-blue-500 to-cyan-500 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></span>
                
                {/* Inner layer (solid bg) */}
                <span className="relative z-10 block bg-[#0E0E0E] group-hover:bg-white group-hover:text-[#1D2460] px-6 py-3 rounded-full border border-white transition-all duration-300">
                    Get Started Now
                </span>
            </button>

        </div>
        </section>
      </section>

      {/* Gambar hitam sebagai footer */}
      <section
        className="w-full bg-no-repeat bg-bottom bg-[length:100%_348px] text-white flex items-center justify-center text-center"
        style={{
          backgroundImage: `url(${blackBg})`,
          height: '348px',
        }}
      >
        <div>
          <p className="font-inter text-base">VeriFlair © 2025</p>
          <div className="font-inter text-base space-x-4 mt-2">
            <a href="#">GitHub</a>
            <span>|</span>
            <a href="#">Internet Identity</a>
            <span>|</span>
            <a href="#">Docs</a>
            <span>|</span>
            <a href="#">Contact</a>
          </div>
        </div>
      </section>
    </div>
  );
}
