module.exports = {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        ink: "#10212b",
        mist: "#eef4f1",
        pine: "#1c6b52",
        amber: "#f2ae49",
        coral: "#d95d39"
      },
      boxShadow: {
        soft: "0 24px 60px rgba(16, 33, 43, 0.08)"
      }
    }
  },
  plugins: []
};
