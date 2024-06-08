pub mod remote;
pub mod resource;
pub mod user_groups;
pub mod variables;

mod file;
mod resources;

fn muted(content: &str) -> String {
  format!("<span class=\"text-muted-foreground\">{content}</span>")
}

fn bold(content: &str) -> String {
  format!("<span class=\"font-bold\">{content}</span>")
}

pub fn colored(content: &str, color: &str) -> String {
  format!("<span class=\"text-{color}-500\">{content}</span>")
}
