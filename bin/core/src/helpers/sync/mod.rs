pub mod remote;
pub mod resource;

mod file;
mod resources;

fn muted(content: &str) -> String {
  format!("<span class=\"text-muted-foreground\">{content}</span>")
}

fn bold(content: &str) -> String {
  format!("<span class=\"font-bold\">{content}</span>")
}

fn colored(content: &str, color: &str) -> String {
  format!("<span class=\"text-{color}-500\">{content}</span>")
}
