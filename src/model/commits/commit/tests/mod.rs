use rstest::rstest;

use crate::model::commits::commit::Commit;
use crate::model::commits::Commits;

#[test]
fn test_get_affected_resources_is_sorted() {
    // Given
    let expected_affected_resources: Vec<String> =
        IntoIterator::into_iter(["LICENSE", "README.md", "src/main.rs"])
            .map(|resource| resource.to_string())
            .collect();
    let commits = Commits {
        commits: vec![
            Commit {
                oid: git2::Oid::zero(),
                affects: IntoIterator::into_iter(["README.md"])
                    .map(|resource| resource.to_string())
                    .collect(),
            },
            Commit {
                oid: git2::Oid::zero(),
                affects: IntoIterator::into_iter(["LICENSE", "src/main.rs"])
                    .map(|resource| resource.to_string())
                    .collect(),
            },
        ],
    };

    // When
    let affected_resources = commits.get_affected_resources();

    // Then
    assert_eq!(
        expected_affected_resources.iter().collect::<Vec<&String>>(),
        affected_resources
    );
}

#[test]
fn test_get_affected_resources_is_unique() {
    // Given
    let expected_affected_resources: Vec<String> =
        IntoIterator::into_iter(["LICENSE", "README.md", "src/main.rs"])
            .map(|resource| resource.to_string())
            .collect();
    let commits = Commits {
        commits: vec![
            Commit {
                oid: git2::Oid::zero(),
                affects: IntoIterator::into_iter(["README.md"])
                    .map(|resource| resource.to_string())
                    .collect(),
            },
            Commit {
                oid: git2::Oid::zero(),
                affects: IntoIterator::into_iter(["LICENSE", "src/main.rs"])
                    .map(|resource| resource.to_string())
                    .collect(),
            },
            Commit {
                oid: git2::Oid::zero(),
                affects: IntoIterator::into_iter(["README.md"])
                    .map(|resource| resource.to_string())
                    .collect(),
            },
            Commit {
                oid: git2::Oid::zero(),
                affects: IntoIterator::into_iter(["src/main.rs"])
                    .map(|resource| resource.to_string())
                    .collect(),
            },
        ],
    };

    // When
    let affected_resources = commits.get_affected_resources();

    // Then
    assert_eq!(
        expected_affected_resources.iter().collect::<Vec<&String>>(),
        affected_resources
    );
}

#[rstest]
// Single file.
#[case(vec!["README.md"], vec!["README.md", "LICENSE", "src/main.rs"])]
// Single file within a directory.
#[case(vec!["src/cli/mod.rs"], vec!["src/cli/mod.rs", "src/main.rs"])]
// Multiple files all match.
#[case(vec![".gitignore", ".dockerignore"], vec!["README.md", ".gitignore", ".dockerignore", "Dockerfile", "src/main.rs"])]
// Multiple files some match.
#[case(vec!["azure-pipelines.yml", "SECURITY.md"], vec!["README.md", ".gitignore", ".dockerignore", "Dockerfile", "SECURITY.md"])]
// Multiple files within a directory all match.
#[case(vec!["src/cli/mod.rs", "src/model/mod.rs"], vec!["src/cli/mod.rs", "src/bin.rs", "src/lib.rs", "src/model/mod.rs", "src/model/commits/mod.rs"])]
// Multiple files within a directory some match.
#[case(vec!["config/BUILD.md", "rdpapplist/rdpapplist_server.h", "WSLGd/Makefile"], vec!["WSLGd/Makefile", "README.md", ".gitignore"])]
// Single directory.
#[case(vec!["docs/"], vec!["src/main.rs", "src/lib.rs", "docs/DEVELOPMENT.md"])]
// Single nested directory.
#[case(vec!["src/model/"], vec!["src/main.rs", "src/lib.rs", "src/model/mod.rs"])]
// Multiple directories all match.
#[case(vec!["examples/", "src/"], vec!["src/main.rs", "src/lib.rs", "example/README.md"])]
// Multiple directories some match.
#[case(vec!["examples/", "docs/"], vec!["src/main.rs", "src/lib.rs", "docs/DEVELOPMENT.md"])]
// Multiple nested directories all match.
#[case(vec!["examples/hello-world/", "src/model"], vec!["src/main.rs", "src/model/mod.rs", "src/lib.rs", "example/hello-world/README.md"])]
// Multiple nested directories some match.
#[case(vec!["monorepo-1/src/", "monorepo-lib/src/"], vec!["monorepo-lib/src/lib.rs", "monorepo-2/src/bin.rs", "example/README.md"])]
fn test_is_affected(#[case] affects: Vec<&str>, #[case] affected_resources: Vec<&str>) {
    // Given
    let affects: Vec<String> = affects
        .into_iter()
        .map(|resource| resource.to_string())
        .collect();
    let commits = Commits {
        commits: affected_resources
            .into_iter()
            .map(|resource| Commit {
                oid: git2::Oid::zero(),
                affects: IntoIterator::into_iter([resource])
                    .map(|resource| resource.to_string())
                    .collect(),
            })
            .collect(),
    };

    // When/Then
    assert!(commits.is_affected(&affects).unwrap());
}

#[rstest]
// Single file.
#[case(vec!["src/lib.rs"], vec!["README.md", "LICENSE", "src/main.rs"])]
// Single file within a directory.
#[case(vec!["src/model/mod.rs"], vec!["src/cli/mod.rs", "src/main.rs"])]
// Multiple files none match.
#[case(vec!["src/lib.rs", "src/bin.rs"], vec!["README.md", ".gitignore", ".dockerignore", "Dockerfile", "src/main.rs"])]
// Multiple files within a directory none match.
#[case(vec!["src/cli/mod.rs", "src/model/mod.rs"], vec!["src/bin.rs", "src/lib.rs", "src/model/commits/mod.rs"])]
// Single directory.
#[case(vec!["docs/"], vec!["src/main.rs", "src/lib.rs", "example/README.md"])]
// Single nested directory.
#[case(vec!["src/model/"], vec!["src/main.rs", "src/lib.rs", "README.md"])]
// Multiple directories none match.
#[case(vec!["examples/", "docs/"], vec!["src/main.rs", "src/lib.rs", "src/model/mod.rs"])]
// Multiple nested directories none match.
#[case(vec!["examples/docs/", "src/model"], vec!["src/main.rs", "src/lib.rs", "example/README.md"])]
fn test_is_not_affected(#[case] affects: Vec<&str>, #[case] affected_resources: Vec<&str>) {
    // Given
    let affects: Vec<String> = affects
        .into_iter()
        .map(|resource| resource.to_string())
        .collect();
    let commits = Commits {
        commits: affected_resources
            .into_iter()
            .map(|resource| Commit {
                oid: git2::Oid::zero(),
                affects: IntoIterator::into_iter([resource])
                    .map(|resource| resource.to_string())
                    .collect(),
            })
            .collect(),
    };

    // When/Then
    assert!(!commits.is_affected(&affects).unwrap());
}
