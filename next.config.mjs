/** @type {import('next').NextConfig} */
import withVideos from "next-videos";

const nextConfig = {
  output: "export",
};

export default {
  ...nextConfig,
  ...withVideos(),
};
