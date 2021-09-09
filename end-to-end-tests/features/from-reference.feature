Feature: A Git reference can be provided as an argument to indicate where to start taking the range of commits from till HEAD, to check if the resources are effected.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-reference is provided as "<from_reference>".
    And the argument --effects is provided as "<effects>".
    Then is effected.


    Examples:
      | repository                              | checkout_commit                          | from_reference | effects                             |
      | https://github.com/xfangfang/Macast.git | f105522c9290dd3202249ff8c8778bebcd3f361b | v0.5           | .github/workflows/build-macast.yaml |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-reference is provided as "<from_reference>".
    And the argument --effects is provided as "<effects>".
    Then is not effected.


    Examples:
      | repository                               | checkout_commit                          | from_reference | effects        |
      | https://gitlab.com/wwn-mode/WWN-Mode.git | 3839e0b2a02687a23aae4fa49d5ec0eea1762f17 | 0.0.2Update    | Units/BUM0.ini |


  Scenario Outline: You can also provide the long name and partial names not just the short name.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-reference is provided as "<from_full_reference>".
    And the argument --effects is provided as "<effects>".
    Then is effected.
    Given the arguments are reset.
    When the argument --from-reference is provided as "<from_partial_reference>".
    And the argument --effects is provided as "<effects>".
    Then is effected.
    Given the arguments are reset.
    When the argument --from-reference is provided as "<from_short_reference>".
    And the argument --effects is provided as "<effects>".
    Then is effected.


    Examples:
      | repository                              | checkout_commit                          | from_full_reference | from_partial_reference | from_short_reference | effects                  |
      | https://github.com/countercept/chainsaw | 9503898cdd1f594c4f19d56ef9ccd6aed60a2408 | refs/tags/v1.0.0    | tags/v1.0.0            | v1.0.0               | .github/workflows/ci.yml |


  Scenario Outline: When you provide an invalid reference a relevant error message is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When the argument --from-reference is provided as "<from_reference>".
    And the --list flag is set.
    Then their is a could not find reference "<from_reference>" error.


    Examples:
      | repository                             | checkout_commit                          | from_reference |
      | https://github.com/FiloSottile/age.git | 8d88096476305f4c5bf764bdea298291bb48ea00 | 1.0.0-rc.3     |
