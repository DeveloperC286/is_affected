Feature: A shortened Git commit hash can be provided as an argument to indicate where to start taking the range of commits from till HEAD, to check if the resources are effected.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --effects is provided as "<effects>".
    Then is effected.
    Given the arguments are reset.
    When the argument --from-commit-hash is provided as "<from_shortened_commit_hash>".
    And the argument --effects is provided as "<effects>".
    Then is effected.


    Examples:
      | repository                               | checkout_commit                          | from_commit_hash                         | effects | from_shortened_commit_hash |
      | https://github.com/SeanDaBlack/AbBOT.git | ae11d60fd2244703c5c22015e6e1c9b021da81af | 98d2d48419e2d1db15302da7dfc1d994fa58c94c | FAQ.md  | 98d2d48                    |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --effects is provided as "<effects>".
    Then is not effected.
    Given the arguments are reset.
    When the argument --from-commit-hash is provided as "<from_shortened_commit_hash>".
    And the argument --effects is provided as "<effects>".
    Then is not effected.


    Examples:
      | repository                                 | checkout_commit                          | from_commit_hash                         | effects   | from_shortened_commit_hash |
      | https://gitlab.com/dcacademy/tutorials.git | a19b133490c6a20803e8cf0a91725812e19ff461 | 90cb36dcf04582ce8eda3002c7d19fe3785e67ca | book.toml | 90cb36d                    |


  Scenario Outline: The short commit hash matches no commit hashes. So an error is printed and it exits unsuccessfully.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-commit-hash is provided as "<from_shortened_commit_hash>".
    And the argument --effects is provided as "<effects>".
    Then their is a could not find shortened commit hash "<from_shortened_commit_hash>" error.


    Examples:
      | repository                                                 | checkout_commit                          | from_shortened_commit_hash | effects   |
      | https://bitbucket.org/GenaroCamele/ejemplos-rust-libro.git | 0c29529c7beeb6c14076e8dc4601e1c0d875c2b7 | 68b4fde0                   | README.md |


  Scenario Outline: The short commit hash is ambiguous, multiple commit hashes match it. So an error is printed and it exits unsuccessfully.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    And the directory is changed to the cloned repository.
    When the argument --from-commit-hash is provided as "<from_shortened_commit_hash>".
    And the argument --effects is provided as "<effects>".
    Then their is a ambiguous shortened commit hash "<from_shortened_commit_hash>" error.


    Examples:
      | repository                                   | checkout_commit                          | from_shortened_commit_hash | effects |
      | https://github.com/GrrrDog/weird_proxies.git | 846b3cefa35c8d15c408d85ca4d059767eb72c2d | 03                         | LICENSE |
