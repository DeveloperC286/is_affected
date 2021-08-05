Feature: A Git commit hash can be provided as an argument to indicate where to start taking the range of commits from till HEAD to lint.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --effects is provided as "<effects>".
    Then is effected.


    Examples:
      | repository                            | checkout_commit                          | from_commit_hash                         | effects    |
      | https://github.com/Lokathor/bytemuck  | e01c6b3a2df2bd1636fa67b73b9392e826c5de33 | fd901461adbca0031d4b717781b0503dc838d15f | Cargo.toml |
      | https://github.com/BurntSushi/walkdir | abf3a15887758e0af54ebca827c7b6f8b311cb45 | bab4066b218dc20a625d405e02433d882237d59c | Cargo.toml |
      | https://github.com/rust-lang/libm     | 89eaba13ebe7044f6e2034dcd90c5c945b7c3a01 | b0f666e1a218740db09d7d6efb6a610be470225c | README.md  |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the argument --effects is provided as "<effects>".
    Then is not effected.


    Examples:
      | repository                           | checkout_commit                          | from_commit_hash                         | effects      |
      | https://github.com/Lokathor/bytemuck | e01c6b3a2df2bd1636fa67b73b9392e826c5de33 | fd901461adbca0031d4b717781b0503dc838d15f | rustfmt.toml |
      | https://github.com/RAUI-labs/raui    | 4d923829bd5c2f5c73b50c64378302fc3a2afcc2 | 50e1041eb2cb2a6b4682d8db92ceb5941fc9c4d0 | README.md    |
      | https://github.com/RAUI-labs/raui    | 4d923829bd5c2f5c73b50c64378302fc3a2afcc2 | 12b7aedceb3b477bc54bdc694dec8dc239212df5 | LICENSE      |