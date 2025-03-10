Feature: A Git reference can be provided as an argument to indicate where to start taking the range of commits from till HEAD, to check if the resources are affected.


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<reference>".
    And the argument --affects is provided as "<affects>".
    Then is affected.


    Examples:
      | repository                              | checkout_commit                          | reference | affects                             |
      | https://github.com/xfangfang/Macast.git | f105522c9290dd3202249ff8c8778bebcd3f361b | v0.5           | .github/workflows/build-macast.yaml |


  Scenario Outline:
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<reference>".
    And the argument --affects is provided as "<affects>".
    Then is not affected.


    Examples:
      | repository                               | checkout_commit                          | reference | affects        |
      | https://github.com/aws-solutions/quota-monitor-for-aws | f0b61d26e1667f0d85bd6bf5344ac8ca7648c29e | v6.2.11 | .github/PULL_REQUEST_TEMPLATE.md |


  Scenario Outline: You can also provide the long name and partial names not just the short name.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<full_reference>".
    And the argument --affects is provided as "<affects>".
    Then is affected.
    Given the arguments are reset.
    When linting from the "<partial_reference>".
    And the argument --affects is provided as "<affects>".
    Then is affected.
    Given the arguments are reset.
    When linting from the "<short_reference>".
    And the argument --affects is provided as "<affects>".
    Then is affected.


    Examples:
      | repository                              | checkout_commit                          | full_reference | affects                  | partial_reference | short_reference |
      | https://github.com/countercept/chainsaw | 9503898cdd1f594c4f19d56ef9ccd6aed60a2408 | refs/tags/v1.0.0    | .github/workflows/ci.yml | tags/v1.0.0            | v1.0.0               |


  Scenario Outline: When you provide an invalid reference a relevant error message is returned.
    Given the repository "<repository>" is cloned and checked out at the commit "<checkout_commit>".
    When linting from the "<reference>".
    And the --list flag is set.
    Then their is a could not find reference "<reference>" error.


    Examples:
      | repository                             | checkout_commit                          | reference |
      | https://github.com/FiloSottile/age.git | 8d88096476305f4c5bf764bdea298291bb48ea00 | 1.0.0-rc.3     |
