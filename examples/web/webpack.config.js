const path = require("path");

module.exports = {
  mode: "production",
  cache: true,
  entry: path.resolve(
    __dirname,
    "..",
    "..",
    "gen",
    "web",
    "gogh.v1_grpc_web_pb.js"
  ),
  output: {
    path: path.resolve(__dirname),
    filename: "bundle.js",
  },
  resolve: {
    modules: [path.resolve(__dirname, "node_modules")],
  },
};
