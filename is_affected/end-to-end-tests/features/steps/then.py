import re
from behave import then

from utilities import execute_is_affected
from assertions import *


@then('is affected.')
def is_affected(context):
    # When
    execute_is_affected(context)

    # Then
    assert context.stderr == ""
    assert_command_successful(context)


@then('is not affected.')
def is_not_affected(context):
    # When
    execute_is_affected(context)

    # Then
    assert_no_output(context)
    assert_command_unsuccessful(context)


@then('their is a could not find commit hash "{commit_hash}" error.')
def then_could_not_find_commit_hash_error(context, commit_hash):
    # Given
    could_not_find_commit_hash_error = f" ERROR is_affected_lib::commits > Can not find a commit with the hash '{commit_hash}'.\n"

    # When/Then
    is_not_affected(context)

    # Then
    assert context.stderr == could_not_find_commit_hash_error


@then(
    'their is a could not find shortened commit hash "{shortened_commit_hash}" error.')
def then_could_not_find_shortened_commit_hash_error(
        context, shortened_commit_hash):
    # Given
    could_not_find_shortened_commit_hash_error = f" ERROR is_affected_lib::commits > No commit hashes start with the provided short commit hash \"{shortened_commit_hash}\".\n"

    # When/Then
    is_not_affected(context)

    # Then
    assert context.stderr == could_not_find_shortened_commit_hash_error


@then(
    'their is a ambiguous shortened commit hash "{shortened_commit_hash}" error.')
def then_ambiguous_shortened_commit_hash_error(context, shortened_commit_hash):
    # Given
    ambiguous_shortened_commit_hash_error = re.compile(
        f"^ ERROR is_affected_lib::commits > Ambiguous short commit hash, the commit hashes [[]({shortened_commit_hash}[a-f0-9]*(, )?)*[]] all start with the provided short commit hash \"{shortened_commit_hash}\".\n$")

    # When/Then
    is_not_affected(context)

    # Then
    assert ambiguous_shortened_commit_hash_error.match(
        context.stderr) is not None


@then('their is a could not find reference "{reference}" error.')
def then_could_not_find_reference_error(context, reference):
    # Given
    could_not_find_reference_error = f" ERROR is_affected_lib::commits > Could not find a reference with the name \"{reference}\".\n"

    # When/Then
    is_not_affected(context)

    # Then
    assert context.stderr == could_not_find_reference_error


@then('their is a missing from argument error.')
def then_missing_from_argument_error(context):
    # Given
    missing_from_argument_error = "error: The following required arguments were not provided:\n" + \
                                  "    <--from-commit-hash <from-commit-hash>|--from-reference <from-reference>>\n" + \
                                  "\n" + \
                                  "USAGE:\n" + \
                                  "    is_affected <--from-commit-hash <from-commit-hash>|--from-reference <from-reference>> <--affects <affects>...|--affects-current-directory|--list>\n" + \
                                  "\n" + \
                                  "For more information try --help\n"

    # When/Then
    is_not_affected(context)

    # Then
    assert context.stderr == missing_from_argument_error


@then('their is a missing output argument error.')
def then_missing_output_argument_error(context):
    # Given
    missing_output_argument_error = "error: The following required arguments were not provided:\n" + \
                                    "    <--affects <affects>...|--affects-current-directory|--list>\n" + \
                                    "\n" + \
                                    "USAGE:\n" + \
                                    "    is_affected <--from-commit-hash <from-commit-hash>|--from-reference <from-reference>> <--affects <affects>...|--affects-current-directory|--list>\n" + \
                                    "\n" + \
                                    "For more information try --help\n"

    # When/Then
    is_not_affected(context)

    # Then
    assert context.stderr == missing_output_argument_error


@then('their is a conflicting from arguments error.')
def then_conflicting_from_arguments_error(context):
    # Given
    conflicting_arguments_end = "\n" + \
        "USAGE:\n" + \
        "    is_affected <--from-commit-hash <from-commit-hash>|--from-reference <from-reference>> <--affects <affects>...|--affects-current-directory|--list>\n" + \
        "\n" + \
        "For more information try --help\n"
    conflicting_from_commit_hash_error = f"error: The argument '--from-commit-hash <from-commit-hash>' cannot be used with one or more of the other specified arguments\n{conflicting_arguments_end}"
    conflicting_from_reference_error = f"error: The argument '--from-reference <from-reference>' cannot be used with one or more of the other specified arguments\n{conflicting_arguments_end}"

    # When/Then
    is_not_affected(context)

    # Then
    assert context.stderr in [
        conflicting_from_commit_hash_error, conflicting_from_reference_error]


@then('their is a conflicting output arguments error.')
def then_conflicting_output_arguments_error(context):
    # Given
    conflicting_arguments_end = "\n" + \
        "USAGE:\n" + \
        "    is_affected <--from-commit-hash <from-commit-hash>|--from-reference <from-reference>> <--affects <affects>...|--affects-current-directory|--list>\n" + \
        "\n" + \
        "For more information try --help\n"
    conflicting_affects_error = f"error: The argument '--affects <affects>...' cannot be used with one or more of the other specified arguments\n{conflicting_arguments_end}"
    conflicting_list_error = f"error: The argument '--list' cannot be used with one or more of the other specified arguments\n{conflicting_arguments_end}"
    conflicting_affects_current_directory_error = f"error: The argument '--affects-current-directory' cannot be used with one or more of the other specified arguments\n{conflicting_arguments_end}"

    # When/Then
    is_not_affected(context)

    # Then
    assert context.stderr in [
        conflicting_affects_error,
        conflicting_list_error,
        conflicting_affects_current_directory_error]


@then('the affected resources listed are "{affected_resources}".')
def then_affected_resources_are(context, affected_resources):
    # When/Then
    is_affected(context)

    # Then
    assert context.stdout == affected_resources.strip().strip('\"').replace("\\n", '\n')
