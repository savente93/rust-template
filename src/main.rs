
#[allow(unused_imports)]
use {{crate_name}}::*;

use color_eyre::eyre::Result;

{% if include_bin %}
mod cli;
use crate::cli::Args;
use clap::Parser;
{% else %}
use tracing_core::Level;
{% endif %}


{% if include_async %}
#[tokio::main]
async {% endif %} fn main() -> Result<()> {
    color_eyre::install()?;

    {% if include_bin  %}
    let args = Args::parse();
    let subscriber = tracing_subscriber::fmt().with_max_level(args.verbose.tracing_level_filer()).finish();
    {%else%}
        let subscriber = tracing_subscriber::fmt().with_max_level(Level::WARN).finish();
    {% endif %}

    tracing::trace!("trace");
    tracing::debug!("debug");
    tracing::info!("info");
    tracing::warn!("warning");
    tracing::error!("error");

    
    // ...
    Ok(())
}


