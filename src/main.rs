
use color_eyre::eyre::Result;

{% if include_bin %}
mod cli;
use crate::cli::Args;
use clap::Parser;
{% else %}
use tracing_core::Level;
{% endif %}


#[allow(clippy::missing_errors_doc)]
{% if include_async %}
#[tokio::main]
async {% endif %} fn main() -> Result<()> {
    color_eyre::install()?;

    {% if include_bin  %}
    let args = Args::parse();
    let _subscriber = tracing_subscriber::fmt().with_max_level(args.verbose.tracing_level_filter()).finish();
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
