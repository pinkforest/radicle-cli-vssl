use radicle_common as common;
use radicle_common::git;

use crate as term;

/// How a comment is to be supplied by the user for a patch or issue on the terminal.
#[derive(Clone, Debug)]
pub enum Comment {
    /// Prompt user to write comment in editor.
    Edit,
    /// Don't leave a comment.
    Blank,
    /// Use the following string as comment.
    Text(String),
}

impl Comment {
    /// Get the comment as a string according to the method.
    pub fn get(self, help: &str) -> String {
        let comment = match self {
            Comment::Edit => term::Editor::new()
                .require_save(true)
                .trim_newlines(true)
                .extension(".markdown")
                .edit(help)
                .unwrap(),
            Comment::Blank => None,
            Comment::Text(c) => Some(c),
        };
        let comment = comment.unwrap_or_default().replace(help, "");
        let comment = comment.trim();

        comment.to_owned()
    }

    pub fn append(&mut self, arg: &str) {
        if let Comment::Text(v) = self {
            v.extend(["\n\n", arg]);
        } else {
            *self = Comment::Text(arg.into());
        };
    }
}

impl Default for Comment {
    fn default() -> Self {
        Self::Edit
    }
}

/// List the given commits in a table.
pub fn list_commits(commits: &[git::Commit]) -> anyhow::Result<()> {
    let mut table = term::Table::default();

    for commit in commits {
        let message = commit
            .summary_bytes()
            .unwrap_or_else(|| commit.message_bytes());
        table.push([
            term::format::secondary(common::fmt::oid(&commit.id())),
            term::format::italic(String::from_utf8_lossy(message)),
        ]);
    }
    table.render();

    Ok(())
}

/// Print commits ahead and behind.
pub fn print_commits_ahead_behind(
    repo: &git::Repository,
    left: git::Oid,
    right: git::Oid,
) -> anyhow::Result<()> {
    let (ahead, behind) = repo.graph_ahead_behind(left, right)?;

    term::info!(
        "{} commit(s) ahead, {} commit(s) behind",
        term::format::positive(ahead),
        if behind > 0 {
            term::format::negative(behind)
        } else {
            term::format::dim(behind)
        }
    );
    Ok(())
}
