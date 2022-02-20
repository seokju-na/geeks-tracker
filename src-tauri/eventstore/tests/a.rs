use git2::Repository;

const GIT_PATH: &str = "tests/fixtures/append/_git";

#[test]
fn should_open_bare_repo() {
    let repo = Repository::open_bare(GIT_PATH).unwrap();
    assert_eq!(repo.is_bare(), true);
}

#[test]
fn aaa() {
    let repo = Repository::open_bare(GIT_PATH).unwrap();
    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();

    let commits = revwalk.filter_map(|id| {
        let id = id.unwrap();
        let commit = repo.find_commit(id).unwrap();

        Some(commit)
    });

    for commit in commits {
        println!("{} {}", commit.message().unwrap(), commit.id());
    }
}
