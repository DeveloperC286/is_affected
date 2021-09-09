Feature: The effected resouces over the specfied range of commits can be listed out.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-commit-hash is provided as "<from_commit_hash>".
    And the --list flag is set.
    Then the effected resources listed are "<effected_resources>".


    Examples:
      | repository                    | checkout_commit                          | from_commit_hash                         | effected_resources                                                              |
      | https://github.com/mgdm/htmlq | 5cae41f69bebd28a6ea6dea25fd8ab66f408f706 | 6f3c358ab83a95d6890461520f77679f49ea3e0e | ".github/workflows/build.yml\nCargo.lock\nCargo.toml\nREADME.md\nsrc/main.rs\n" |
