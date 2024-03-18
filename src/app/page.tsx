import FullScreenVideo from "@/components/video";
import Link from "next/link";

export default function Home() {
  return (
    <main>
      <FullScreenVideo />
      <div className="absolute top-0 left-0 right-0 bottom-0 flex items-center justify-center">
        <div className="text-center">
          <h1 className="mb-4 text-5xl font-bold text-black">Welcome</h1>
          <p className="text-3xl text-gray-800 p-4">
            Click below to get Started
          </p>
          <button
            type="button"
            className="rounded-full bg-logoGreen px-4 py-2.5 text-l font-semibold text-white shadow-sm hover:text-xl focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
          >
            <Link href="/test">Get Started</Link>
          </button>
        </div>
      </div>
    </main>
  );
}
