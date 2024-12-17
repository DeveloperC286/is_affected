Feature: Check if the current directory contains any of the affected resources within the range of the commits.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<commit_hash>".
    And the argument --affects is provided as "<affects>".
    Then is affected.
    Given the arguments are reset.
    And within the directory "<directory>" inside the cloned repository.
    When linting from the "<commit_hash>".
    And the --affects-current-directory flag is set.
    Then is affected.


    Examples:
      | repository                                    | checkout_commit                          | commit_hash                         | affects    | directory |
      | https://gitlab.com/thelamer/radarr-mirror.git | 746d788889ed4d0bbaae980afb6c86ec6651dea1 | 3d3be90eb1f3277c6fd53e3dee434052f08bd0dc | ^root/etc/ | root/etc/ |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<commit_hash>".
    And the argument --affects is provided as "<affects>".
    Then is not affected.
    Given the arguments are reset.
    And within the directory "<directory>" inside the cloned repository.
    When linting from the "<commit_hash>".
    And the --affects-current-directory flag is set.
    Then is not affected.


    Examples:
      | repository                                 | checkout_commit                          | commit_hash                         | affects | directory |
      | https://github.com/woodruffw/toml2json.git | 1365f3d0ec8617ca880131f45c14977b525167b9 | fc652de8380d89fd28c3d46a8b86ccb74443c1b0 | ^src/   | src/      |
