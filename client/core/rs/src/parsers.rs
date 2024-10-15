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
      let line = if let Some(line) = line.strip_prefix(['"', '\'']) {
        line.strip_suffix(['"', '\'']).unwrap_or(line)
      } else {
        line
      };
      // Remove any preceding '"' (from yaml list) (wrapping quotes open)
      let (key, value) = line
        .split_once(['=', ':'])
        .with_context(|| {
          format!(
            "line {i} missing assignment character ('=' or ':')"
          )
        })
        .map(|(key, value)| {
          let value = value.trim();
          // Remove wrapping quotes around value
          let value =
            if let Some(value) = value.strip_prefix(['"', '\'']) {
              value.strip_suffix(['"', '\'']).unwrap_or(value)
            } else {
              value
            };
          (key.trim().to_string(), value.trim().to_string())
        })?;
      anyhow::Ok((key, value))
    })
    .collect::<anyhow::Result<Vec<_>>>()
}

/// Parses commands out of multiline string
/// and chains them together with '&&'
///
/// Supports full line and end of line comments, and escaped newlines.
///
/// ## Example:
/// ```sh
/// # comments supported
/// sh ./shell1.sh # end of line supported
/// sh ./shell2.sh
///
/// # escaped newlines supported
/// curl --header "Content-Type: application/json" \
///   --request POST \
///   --data '{"key": "value"}' \
///   https://destination.com
///
/// # print done
/// echo done
/// ```
/// becomes
/// ```sh
/// sh ./shell1.sh && sh ./shell2.sh && {long curl command} && echo done
/// ```
pub fn parse_multiline_command(command: impl AsRef<str>) -> String {
  command
    .as_ref()
    // Remove comments and join back
    .split('\n')
    .map(str::trim)
    .filter(|line| !line.is_empty() && !line.starts_with('#'))
    .filter_map(|line| line.split(" #").next())
    .collect::<Vec<_>>()
    .join("\n")
    // Remove escaped newlines
    .split(" \\")
    .map(str::trim)
    .fold(String::new(), |acc, el| acc + " " + el)
    // Then final split by newlines and join with &&
    .split('\n')
    .map(str::trim)
    .filter(|line| !line.is_empty() && !line.starts_with('#'))
    .filter_map(|line| line.split(" #").next())
    .map(str::trim)
    .collect::<Vec<_>>()
    .join(" && ")
}