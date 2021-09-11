use crate::model::commits::commit::Commit;
use crate::model::commits::Commits;

use rstest::rstest;

#[test]
fn test_get_effected_resources_is_sorted() {
    // Given
    let expected_effected_resources: Vec<String> =
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
    let effected_resources = commits.get_effected_resources();

    // Then
    assert_eq!(
        expected_effected_resources.iter().collect::<Vec<&String>>(),
        effected_resources
    );
}

#[test]
fn test_get_effected_resources_is_unique() {
    // Given
    let expected_effected_resources: Vec<String> =
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
    let effected_resources = commits.get_effected_resources();

    // Then
    assert_eq!(
        expected_effected_resources.iter().collect::<Vec<&String>>(),
        effected_resources
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
fn test_is_effected(#[case] effects: Vec<&str>, #[case] effected_resources: Vec<&str>) {
    // Given
    let effects: Vec<String> = effects
        .into_iter()
        .map(|resource| resource.to_string())
        .collect();
    let commits = Commits {
        commits: effected_resources
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
    assert!(commits.is_effected(&effects));
}
