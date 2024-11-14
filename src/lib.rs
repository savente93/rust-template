use anyhow::Result;
pub fn dummy() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_test() -> Result<()> {
        Ok(())
    }
}
