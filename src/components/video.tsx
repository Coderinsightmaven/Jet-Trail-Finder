

export default function FullScreenVideo() {
    return (
        <div className="relative w-full h-screen overflow-hidden">
        <video
          src="/SlideShow.mp4"
          autoPlay
          muted
          loop
          className="object-cover w-full h-full"
        />
        </div>
    )
}