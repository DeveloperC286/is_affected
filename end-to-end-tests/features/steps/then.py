import re

from behave import then

from util import execute_is_effected


@then('is effected.')
def is_effected(context):
    execute_is_effected(context)

    assert context.stdout == ""
    assert context.stderr == ""
    assert int(context.exit_code) == 0


@then('is not effected.')
def is_not_effected(context):
    execute_is_effected(context)

    assert context.stdout == ""
    assert context.stderr == ""
    assert int(context.exit_code) != 0


@then('their is a could not find commit hash "{commit_hash}" error.')
def then_could_not_find_commit_hash(context, commit_hash):
    execute_is_effected(context)
    could_not_find_commit_hash_error = " ERROR is_effected::model::commits > Can not find commit hash '" + \
                                       commit_hash + "' on the Git revision walker.\n"

    assert context.stdout == ""
    assert int(context.exit_code) != 0
    assert context.stderr == could_not_find_commit_hash_error


@then(
    'their is a could not find shortened commit hash "{shortened_commit_hash}" error.')
def then_could_not_find_shortened_commit_hash(context, shortened_commit_hash):
    execute_is_effected(context)
    could_not_find_shortened_commit_hash = " ERROR is_effected::model::commits > No actual commit hashes start with the provided short commit hash \"" + \
        shortened_commit_hash + "\".\n"

    assert context.stdout == ""
    assert int(context.exit_code) != 0
    assert context.stderr == could_not_find_shortened_commit_hash


@then(
    'their is a ambiguous shortened commit hash "{shortened_commit_hash}" error.')
def then_could_not_find_shortened_commit_hash(context, shortened_commit_hash):
    execute_is_effected(context)
    ambiguous_shortened_commit_hash = re.compile(
        '^ ERROR is_effected::model::commits > Ambiguous short commit hash, the commit hashes [[](' +
        shortened_commit_hash +
        '[a-f0-9]*(, )?)*[]] all start with the provided short commit hash "' +
        shortened_commit_hash +
        '".\n$')

    assert context.stdout == ""
    assert int(context.exit_code) != 0
    assert ambiguous_shortened_commit_hash.match(context.stderr) is not None


@then('their is a could not find reference "{reference}" error.')
def then_could_not_find_reference(context, reference):
    execute_is_effected(context)
    could_not_find_reference_error = " ERROR is_effected::model::commits > Could not find a reference with the name \"" + reference + "\".\n"

    assert context.stdout == ""
    assert int(context.exit_code) != 0
    assert context.stderr == could_not_find_reference_error


@then('their is a missing from argument error.')
def then_missing_from_argument_error(context):
    execute_is_effected(context)
    missing_from_argument_error = "error: The following required arguments were not provided:\n" + \
        "    <--from-commit-hash <from-commit-hash>|--from-reference <from-reference>>\n" + \
        "\n" + \
        "USAGE:\n" + \
        "    is_effected <--from-commit-hash <from-commit-hash>|--from-reference <from-reference>> <--effects <effects>...|--list>\n" + \
        "\n" + \
        "For more information try --help\n"

    assert context.stdout == ""
    assert int(context.exit_code) != 0
    assert context.stderr == missing_from_argument_error


@then('their is a missing output argument error.')
def then_missing_output_argument_error(context):
    execute_is_effected(context)
    missing_output_argument_error = "error: The following required arguments were not provided:\n" + \
        "    <--effects <effects>...|--list>\n" + \
        "\n" + \
        "USAGE:\n" + \
        "    is_effected <--from-commit-hash <from-commit-hash>|--from-reference <from-reference>> <--effects <effects>...|--list>\n" + \
        "\n" + \
        "For more information try --help\n"

    assert context.stdout == ""
    assert int(context.exit_code) != 0
    assert context.stderr == missing_output_argument_error


@then('their is a conflicting from arguments error.')
def then_conflicting_from_arguments_error(context):
    execute_is_effected(context)
    conflicting_from_arguments_error_1 = "error: The argument '--from-commit-hash <from-commit-hash>' cannot be used with one or more of the other specified arguments\n" \
        "\n" + \
        "USAGE:\n" + \
        "    is_effected <--from-commit-hash <from-commit-hash>|--from-reference <from-reference>> <--effects <effects>...|--list>\n" + \
        "\n" + \
        "For more information try --help\n"
    conflicting_from_arguments_error_2 = "error: The argument '--from-reference <from-reference>' cannot be used with one or more of the other specified arguments\n" \
        "\n" + \
        "USAGE:\n" + \
        "    is_effected <--from-commit-hash <from-commit-hash>|--from-reference <from-reference>> <--effects <effects>...|--list>\n" + \
        "\n" + \
        "For more information try --help\n"

    assert context.stdout == ""
    assert int(context.exit_code) != 0
    assert context.stderr == conflicting_from_arguments_error_1 or context.stderr == conflicting_from_arguments_error_2


@then('their is a conflicting output arguments error.')
def then_conflicting_output_arguments_error(context):
    execute_is_effected(context)
    conflicting_output_arguments_error_1 = "error: The argument '--effects <effects>...' cannot be used with one or more of the other specified arguments\n" \
        "\n" + \
        "USAGE:\n" + \
        "    is_effected <--from-commit-hash <from-commit-hash>|--from-reference <from-reference>> <--effects <effects>...|--list>\n" + \
        "\n" + \
        "For more information try --help\n"
    conflicting_output_arguments_error_2 = "error: The argument '--list' cannot be used with one or more of the other specified arguments\n" \
        "\n" + \
        "USAGE:\n" + \
        "    is_effected <--from-commit-hash <from-commit-hash>|--from-reference <from-reference>> <--effects <effects>...|--list>\n" + \
        "\n" + \
        "For more information try --help\n"

    assert context.stdout == ""
    assert int(context.exit_code) != 0
    assert context.stderr == conflicting_output_arguments_error_1 or context.stderr == conflicting_output_arguments_error_2


@then('the effected resources listed are "{effected_resources}".')
def then_could_not_find_reference(context, effected_resources):
    execute_is_effected(context)

    assert context.stderr == ""
    assert int(context.exit_code) == 0
    assert context.stdout == effected_resources.strip().strip('\"').replace("\\n", '\n')
