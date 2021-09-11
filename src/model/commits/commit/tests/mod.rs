use crate::model::commits::commit::Commit;
use crate::model::commits::Commits;

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
