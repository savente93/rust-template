
#[allow(unused_imports)]
use {{crate_name}}::*;

use color_eyre::eyre::Result;


{% if include_async %}
#[tokio::main]
async {% endif %} fn main() -> Result<()> {
    color_eyre::install()?;

    // ...
    Ok(())
}


