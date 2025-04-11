
let handle = variable::get("handle").to_lower();
let project_name = variable::get("project-name").to_lower();
system::command("git", ["add", "."]);
system::command("git", ["commit", "-m", "'Initial Commit'"]);
system::command("git", ["remote", "add", "origin", `git@github.com:${handle}/${project_name}.git`"]);
