use anyhow::Context;

pub fn parse_key_value_list(
  input: &str,
) -> anyhow::Result<Vec<(String, String)>> {
  let trimmed = input.trim();
  if trimmed.is_empty() {
    return Ok(Vec::new());
  }
  trimmed
    .split('\n')
    .map(|line| line.trim())
    .enumerate()
    .filter(|(_, line)| {
      !line.is_empty()
        && !line.starts_with('#')
        && !line.starts_with("//")
    })
    .map(|(i, line)| {
      let line = line
        // Remove end of line comments
        .split_once(" #")
        .unwrap_or((line, ""))
        .0
        .trim()
        // Remove preceding '-' (yaml list)
        .trim_start_matches('-')
        .trim();
      // Remove wrapping quotes (from yaml list)
      let line = if let Some(line) = line.strip_prefix('"') {
        line.strip_suffix('"').unwrap_or(line)
      } else {
        line
      };
      // Remove any preceding '"' (from yaml list) (wrapping quotes open)
      let (key, value) = line
        .split_once(['=', ':', ' '])
        .with_context(|| {
          format!(
            "line {i} missing assignment character ('=' or ':')"
          )
        })
        .map(|(key, value)| {
          let value = value.trim();
          // Remove wrapping quotes around value
          if let Some(value) = value.strip_prefix('"') {
            value.strip_suffix('"').unwrap_or(value)
          } else {
            value
          };
          (key.trim().to_string(), value.trim().to_string())
        })?;
      anyhow::Ok((key, value))
    })
    .collect::<anyhow::Result<Vec<_>>>()
}
