
// format the created project
system::command("cargo", ["fmt", "--all"]);
system::command("taplo", ["fmt", "Cargo.toml"]);
