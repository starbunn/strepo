use std::fs;
use std::process::Command;

pub fn setup_js(workspace_name: String) {
	fs::create_dir(".vscode").err();
	let code_workspace = "
	{
	\"folders\": [
		{
			\"name\": \"root\",
			\"path\": \".\"
		}
	],
	\"settings\": {{}},
	\"extensions\": {
		\"recommendations\": [
			\"esbenp.prettier-vscode\",
			\"hookyqr.beautify\",
			\"dbaeumer.vscode-eslint\"
		]
	}
}
";
	let name = format!(
		"{workspace_name}.code-workspace",
		workspace_name = workspace_name
	);
	fs::write(name, code_workspace).err();
	Command::new("npm")
		.arg("install")
		.arg("--save-dev")
		.arg("@commitlint/{{cli,config-conventional}}");
	Command::new("echo")
		.arg("module.exports = {{extends: ['@commitlint/config-conventional']}}")
		.arg(">")
		.arg("commitlint.config.js");
	Command::new("npm")
		.arg("install")
		.arg("husky")
		.arg("--save-dev");
	Command::new("npx").arg("husky").arg("install");
	Command::new("npx")
		.arg("husky")
		.arg("add")
		.arg(".husky/commit-msg")
		.arg("'npx --no -- commitlint --edit $1'");
}
