use color_eyre::eyre::Result;

// okay here, it's just a stub
#[allow({% if include_async %} clippy::unused_async, {% endif %} clippy::missing_errors_doc)]
pub {% if include_async %} async {% endif %} fn dummy() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    {% if include_async %}
    #[tokio::test]
    async
    {%- else -%}
    #[test]
    {% endif %}
    fn dummy_test() -> Result<()> {
        Ok(())
    }
}
