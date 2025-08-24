#!/usr/bin/env node

console.log("🦉 Qipi is not yet available.");
console.log("💡 The package is reserved as a placeholder on npm.");
console.log("📦 Check https://github.com/qipkg/qipi for updates.");

/*
const { existsSync, chmodSync, mkdirSync, createWriteStream } = require("fs");
const https = require("https");
const path = require("path");
const os = require("os");

const version = require("../package.json").version;

function getPlatform() {
	const type = os.type();
	const arch = os.arch();

	if (type === "Windows_NT") {
		if (arch === "x64") return "x86_64-pc-windows-msvc";
		if (arch === "arm64") return "aarch64-pc-windows-msvc";
		if (arch === "ia32") return "i686-pc-windows-msvc";
	}

	if (type === "Linux") {
		if (arch === "x64") return "x86_64-unknown-linux-gnu";
		if (arch === "arm64") return "aarch64-unknown-linux-gnu";
		if (arch === "ia32") return "i686-unknown-linux-gnu";
	}

	if (type === "Darwin") {
		if (arch === "arm64") return "aarch64-apple-darwin";
		if (arch === "x64") return "x86_64-apple-darwin";
	}

	throw new Error(`Unsupported platform/arch: ${type} ${arch}`);
}

function getDownloadUrl(platform) {
	const ext = platform.includes("windows") ? ".exe" : "";
	return `https://github.com/qipkg/qipi/releases/download/v${version}/qp-${platform}${ext}`;
}

async function downloadBinary() {
	const platform = getPlatform();
	const url = getDownloadUrl(platform);
	const binDir = path.join(__dirname, "bin");
	const binPath = path.join(
		binDir,
		"qp" + (platform.includes("windows") ? ".exe" : ""),
	);

	console.log("🦉 Installing Qipi (qp)...");
	console.log(`📥 Downloading from: ${url}`);

	if (!existsSync(binDir)) {
		mkdirSync(binDir, { recursive: true });
	}

	return new Promise((resolve, reject) => {
		const file = createWriteStream(binPath);
		const req = https.get(url, (response) => {
			if (response.statusCode === 200) {
				response.pipe(file);
				file.on("finish", () => {
					file.close();
					try {
						chmodSync(binPath, 0o755);
						console.log("✅ Qipi installed successfully!");
						console.log(`📍 Binary location: ${binPath}`);
						console.log("🚀 Try running: qp --help");
						resolve();
					} catch (err) {
						reject(err);
					}
				});
			} else if (response.statusCode === 302 || response.statusCode === 301) {
				https
					.get(response.headers.location, (redirectResponse) => {
						redirectResponse.pipe(file);
						file.on("finish", () => {
							file.close();
							chmodSync(binPath, 0o755);
							console.log("✅ Qipi installed successfully!");
							resolve();
						});
					})
					.on("error", reject);
			} else if (response.statusCode === 404) {
				reject(
					new Error(
						`No binary available for platform: ${platform}\n💡 Please check https://github.com/qipkg/qipi/releases or install via Cargo: cargo install qipi`,
					),
				);
			} else {
				reject(
					new Error(
						`Failed to download: ${response.statusCode} ${response.statusMessage}`,
					),
				);
			}
		});

		req.setTimeout(30000, () => {
			req.destroy(new Error("Request timed out"));
		});

		req.on("error", reject);
		file.on("error", reject);
	});
}

async function main() {
	try {
		await downloadBinary();
	} catch (error) {
		console.error("❌ Installation failed:", error.message);
		console.error(
			"💡 You can try installing via Cargo instead: cargo install qipi",
		);
		process.exit(1);
	}
}

if (require.main === module) {
	main();
}
*/
