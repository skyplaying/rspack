var webpack = require("@rspack/core");
/** @type {import("@rspack/core").Configuration} */
module.exports = {
	plugins: [
		new webpack.DllReferencePlugin({
			name: "function(id) { return {default: 'ok'}; }",
			scope: "dll",
			content: {
				"./module": {
					id: 1,
					buildMeta: {
						exportsType: "namespace",
						providedExports: ["default"]
					}
				}
			}
		}),
		new webpack.optimize.ModuleConcatenationPlugin()
	]
};
