let include_bin = variable::get("include_bin");
if !include_bin {
	file::delete("src/cli.rs");
}

let include_bench = variable::get("include_bench");
if !include_bench {
	file::delete("benches/");
}
