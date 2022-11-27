use std::env::current_dir;

use git2::{IndexAddOption, Repository, Signature, Tree};

use crate::version::Version;

/// Performs Git related operations in the crate's repository
pub struct Git {
    email: String,
    name: String,
    branch: String,
    repo: Repository,
}

impl Git {
    /// Opens the Git repository in the current working directory and uses the
    /// provided `email`, `name` and `branch` to perform Git operations like
    /// `commit` and `tag`.
    pub fn open(branch: &str, email: &str, name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let cwd = current_dir()?;
        let repo = Repository::open(cwd)?;

        Ok(Self {
            email: email.into(),
            name: name.into(),
            branch: branch.into(),
            repo,
        })
    }

    /// Creates a commit with instance's Email, Name and Branch with the
    /// taggging tree set. This means, adding `Cargo.toml` and `Cargo.lock`.
    pub fn commit(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let signature = self.signature()?;
        let head = self.repo.head()?.peel_to_commit()?;
        let tree = self.tagging_tree()?;

        self.repo.commit(
            Some(&format!("refs/heads/{}", &self.branch)),
            &signature,
            &signature,
            message,
            &tree,
            &[&head],
        )?;

        Ok(())
    }

    /// Creates a Git Tag with the provided `Version`
    pub fn tag(&self, version: &Version, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let tagger = self.signature()?;
        let head = self.repo.head()?.peel_to_commit()?;
        let obj = head.as_object();

        self.repo
            .tag(&version.to_string(), obj, &tagger, message, false)?;

        Ok(())
    }

    /// Creates a `Signature` using the instance's `email` and `name` along with
    /// the current time
    fn signature(&self) -> Result<Signature, Box<dyn std::error::Error>> {
        let signature = Signature::now(&self.name, &self.email)?;

        Ok(signature)
    }

    /// Creates a Git tree by adding all the files in the current repository
    fn tagging_tree(&self) -> Result<Tree, Box<dyn std::error::Error>> {
        let mut index = self.repo.index()?;

        index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;

        let tree = index.write_tree()?;
        let tree = self.repo.find_tree(tree)?;

        Ok(tree)
    }
}
