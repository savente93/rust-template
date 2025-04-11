
// it's a new project so we can safely upgrade all dependencies breaking
// just prints stuff we don't care about so just drop it
// but if we don't catch it it will error
let foo = system::command("just", ["newest"]);
