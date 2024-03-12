export default function LoginLayout({
    children, // will be a page or nested layout
  }: {
    children: React.ReactNode
  }) {
    return (
      <section className="h-screen flex flex-col">
        {children}
      </section>
    )
  }