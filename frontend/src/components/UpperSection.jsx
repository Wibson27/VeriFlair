import frameText from '../assets/image/fontupper.png';
import card1 from '../assets/image/card1.png';
import card2 from '../assets/image/card2.png';
import card3 from '../assets/image/card3.png';
import card4 from '../assets/image/card4.png';
import card5 from '../assets/image/card5.png';

export default function UpperSection() {
  return (
    <section
      className="w-full min-h-screen bg-[#0E0E0E] text-white flex flex-col items-center justify-center px-6 snap-start"
    >
      {/* Judul dan deskripsi sebagai gambar */}
      <img
        src={frameText}
        alt="Revolutionary Features"
        className="w-[600px] mb-12"
      />

        <div className="space-y-8">
        {/* Baris pertama: 3 card */}
        <div className="flex justify-center gap-6 flex-wrap">
            {[card1, card2, card3].map((card, i) => (
            <img
                key={i}
                src={card}
                alt={`card${i + 1}`}
                className="w-[200px] rounded-xl transition-all duration-300 shadow-[0_0_20px_#4f46e5] hover:shadow-[0_0_40px_#6366f1]"
            />
            ))}
        </div>

        {/* Baris kedua: 2 card */}
        <div className="flex justify-center gap-6">
            {[card4, card5].map((card, i) => (
            <img
                key={i}
                src={card}
                alt={`card${i + 4}`}
                className="w-[200px] rounded-xl transition-all duration-300 shadow-[0_0_20px_#4f46e5] hover:shadow-[0_0_40px_#6366f1]"
            />
            ))}
        </div>
        </div>



      {/* <img
        src={card}
        alt="card"
        className="w-[600px] shadow-[0_0_40px_#3b82f6] hover:shadow-[0_0_60px_#60a5fa] transition-shadow duration-500 rounded-xl"
        /> */}

    </section>
  );
}
