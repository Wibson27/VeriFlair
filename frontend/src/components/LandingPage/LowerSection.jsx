import bgImage from '../../assets/image/LandingPage/bglower.png';
import frameText from '../../assets/image/LandingPage/fontlower.png';
import nftImage from '../../assets/image/LandingPage/nft.png';

export default function LowerSection() {
  return (
    <section
    className="w-full text-white flex flex-col items-center justify-center text-center px-6"
    style={{
        backgroundImage: `url(${bgImage})`,
        backgroundRepeat: 'no-repeat',
        backgroundSize: '100% 100%',
        backgroundPosition: 'top',
        height: '1035px', // fix height sesuai gambar
        width: '100%',
    }}
    >
      {/* Judul dan deskripsi sebagai gambar */}
      <img src={frameText} alt="Earn Language Badges" className="w-full max-w-2xl mb-8" />

      {/* Gambar NFT */}
      <img
        src={nftImage}
        alt="NFT Badge"
        className="w-[300px] md:w-[350px] transition-transform hover:scale-105 duration-300 drop-shadow-[0_0_40px_#3b82f6]"
      />
    </section>
  );
}
