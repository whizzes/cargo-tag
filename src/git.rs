use std::env::current_dir;

use git2::{BranchType, IndexAddOption, Repository, Signature, Tree};

pub struct Git {
    email: String,
    name: String,
    branch: String,
    repo: Repository,
}

impl Git {
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

    /// Creates a `Signature` using the instance's `email` and `name` along with
    /// the current time
    fn signature(&self) -> Result<Signature, Box<dyn std::error::Error>> {
        let signature = Signature::now(&self.name, &self.email)?;

        Ok(signature)
    }

    /// Creates a Git tree where `Cargo.toml` and `Cargo.lock` are addedd to
    /// the commit tree.
    fn tagging_tree(&self) -> Result<Tree, Box<dyn std::error::Error>> {
        let mut index = self.repo.index()?;

        index.add_all(
            ["Cargo.toml", "Cargo.lock"].iter(),
            IndexAddOption::DEFAULT,
            None,
        )?;

        let tree = index.write_tree()?;
        let tree = self.repo.find_tree(tree)?;

        Ok(tree)
    }
}
