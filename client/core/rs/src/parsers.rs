use anyhow::Context;

pub const QUOTE_PATTERN: &[char] = &['"', '\''];

/// Parses a list of key value pairs from a multiline string
///
/// Example source:
/// ```text
/// # Supports comments
/// KEY_1 = value_1 # end of line comments
///
/// # Supports string wrapped values
/// KEY_2="value_2"
/// 'KEY_3 = value_3'
///
/// # Also supports yaml list formats
/// - KEY_4: 'value_4'
/// - "KEY_5=value_5"
///
/// # Wrapping outer quotes are removed while inner quotes are preserved
/// "KEY_6 = 'value_6'"
/// ```
///
/// Note this preserves the wrapping string around value.
/// Writing environment file should format the value exactly as it comes in,
/// including the given wrapping quotes.
///
/// Returns:
/// ```text
/// [
///   ("KEY_1", "value_1"),
///   ("KEY_2", "\"value_2\""),
///   ("KEY_3", "value_3"),
///   ("KEY_4", "'value_4'"),
///   ("KEY_5", "value_5"),
///   ("KEY_6", "'value_6'"),
/// ]
/// ```
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
        .trim_start_matches("- ")
        .trim();
      let (key, value) = line
        .split_once(['=', ':'])
        .with_context(|| {
          format!(
            "line {i} missing assignment character ('=' or ':')"
          )
        })
        .map(|(key, value)| {
          let key = key.trim();
          let value = value.trim();

          // Remove wrapping quotes when around key AND value
          let (key, value) = if key.starts_with(QUOTE_PATTERN)
            && !key.ends_with(QUOTE_PATTERN)
            && value.ends_with(QUOTE_PATTERN)
          {
            (
              key.strip_prefix(QUOTE_PATTERN).unwrap().trim(),
              value.strip_suffix(QUOTE_PATTERN).unwrap().trim(),
            )
          } else {
            (key, value)
          };

          (key.to_string(), value.to_string())
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

/// Parses a list of strings from a comment seperated and multiline string
///
/// Example source:
/// ```text
/// # supports comments
/// path/to/file1 # comment1
/// path/to/file2
///
/// # also supports comma seperated values
/// path/to/file3,path/to/file4
/// ```
///
/// Returns:
/// ```text
/// ["path/to/file1", "path/to/file2", "path/to/file3", "path/to/file4"]
/// ```
pub fn parse_string_list(source: impl AsRef<str>) -> Vec<String> {
  source
    .as_ref()
    .split('\n')
    .map(str::trim)
    .filter(|line| !line.is_empty() && !line.starts_with('#'))
    .filter_map(|line| line.split(" #").next())
    .flat_map(|line| line.split(','))
    .map(str::trim)
    .filter(|entry| !entry.is_empty())
    .map(str::to_string)
    .collect()
}
