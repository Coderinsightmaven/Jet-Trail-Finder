import SlideShowVideo from "../../public/SlideShow.mp4"

export default function FullScreenVideo() {
    return (
        <div className="relative w-full h-screen overflow-hidden">
        <video
          src={SlideShowVideo}
          autoPlay
          muted
          loop
          className="object-cover w-full h-full"
        />
        </div>
    )
}