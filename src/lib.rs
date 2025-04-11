use color_eyre::eyre::Result;

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
