Feature: A shortened Git commit hash can be provided as an argument to indicate where to start taking the range of commits from till HEAD, instead of a full Git commit hash.


  Scenario Outline: A shortened and full Git commit hash can be used interchangeably.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --affects is provided as "<affects>".
    Then is affected.
    Given the arguments are reset.
    When the argument --from-commit-hash is provided as "<from_shortened_commit_hash>".
    And the argument --affects is provided as "<affects>".
    Then is affected.


    Examples:
      | repository                               | checkout_commit                          | from_commit_hash                         | affects | from_shortened_commit_hash |
      | https://github.com/SeanDaBlack/AbBOT.git | ae11d60fd2244703c5c22015e6e1c9b021da81af | 98d2d48419e2d1db15302da7dfc1d994fa58c94c | FAQ.md  | 98d2d48                    |


  Scenario Outline: A shortened and full Git commit hash can be used interchangeably.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --affects is provided as "<affects>".
    Then is not affected.
    Given the arguments are reset.
    When the argument --from-commit-hash is provided as "<from_shortened_commit_hash>".
    And the argument --affects is provided as "<affects>".
    Then is not affected.


    Examples:
      | repository                                 | checkout_commit                          | from_commit_hash                         | affects   | from_shortened_commit_hash |
      | https://gitlab.com/dcacademy/tutorials.git | a19b133490c6a20803e8cf0a91725812e19ff461 | 90cb36dcf04582ce8eda3002c7d19fe3785e67ca | book.toml | 90cb36d                    |


  Scenario Outline: The shortened Git commit hash has no matches, so an error is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_shortened_commit_hash>".
    And the argument --affects is provided as "<affects>".
    Then their is a could not find shortened commit hash "<from_shortened_commit_hash>" error.


    Examples:
      | repository                                                 | checkout_commit                          | from_shortened_commit_hash | affects   |
      | https://bitbucket.org/GenaroCamele/ejemplos-rust-libro.git | 0c29529c7beeb6c14076e8dc4601e1c0d875c2b7 | 68b4fde0                   | README.md |


  Scenario Outline: The shortened Git commit hash is ambiguous as multiple commit hashes match it, so an error is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_shortened_commit_hash>".
    And the argument --affects is provided as "<affects>".
    Then their is a ambiguous shortened commit hash "<from_shortened_commit_hash>" error.


    Examples:
      | repository                                   | checkout_commit                          | from_shortened_commit_hash | affects |
      | https://github.com/GrrrDog/weird_proxies.git | 846b3cefa35c8d15c408d85ca4d059767eb72c2d | 03                         | LICENSE |
