use std::fmt::Display;

#[cfg(windows)]
const EOL: &str = "\r\n";
#[cfg(not(windows))]
const EOL: &str = "\n";

#[derive(Debug, PartialEq, Eq)]
pub struct CommitMessage {
  pub subject: String,
  pub body: String,
}

impl<T> From<T> for CommitMessage
where
  T: AsRef<str>,
{
  fn from(message: T) -> Self {
    let lines: Vec<_> = message.as_ref().split(EOL).collect();
    let subject = lines.first().unwrap_or(&"").to_string();
    let body = if lines.len() > 2 {
      lines[2..].join(EOL)
    } else {
      "".to_owned()
    };

    Self { subject, body }
  }
}

impl Display for CommitMessage {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}{}{}", self.subject, EOL, EOL, self.body)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_parse_commit_message() {
    let subject = "hello";
    let body = format!("word1{}word2{}word3", EOL, EOL);
    let message = format!("{}{}{}{}", subject, EOL, EOL, body);
    let commit_message = CommitMessage::from(message);

    assert_eq!(commit_message.subject, subject);
    assert_eq!(commit_message.body, body);
  }

  #[test]
  fn should_parse_subject_only_commit_message() {
    let commit_message = CommitMessage::from("subject".to_string());

    assert_eq!(commit_message.subject, "subject");
    assert_eq!(commit_message.body, "");
  }

  #[test]
  fn should_message_into() {
    let commit_message: CommitMessage = format!("subject{}{}body", EOL, EOL).into();

    assert_eq!(commit_message.subject, "subject");
    assert_eq!(commit_message.body, "body");
  }

  #[test]
  fn should_convert_to_string() {
    let message = format!("subject{}{}body", EOL, EOL);
    let commit_message: CommitMessage = message.clone().into();

    assert_eq!(commit_message.to_string(), message);
  }
}
