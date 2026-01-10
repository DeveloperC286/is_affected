import re
from behave import then

from utilities import execute_is_affected
from assertions import (
    assert_affected_resources,
    assert_command_successful,
    assert_command_unsuccessful,
    assert_error_contains,
    assert_error_equals,
    assert_error_is_one_of,
    assert_error_matches_regex,
    assert_no_errors,
    assert_no_output,
)


@then('is affected.')
def assert_is_affected(context):
    # When
    result = execute_is_affected(context)

    # Then
    assert_no_errors(result)
    assert_command_successful(result)
    return result


@then('the affected resources listed are "{affected_resources}".')
def assert_affected_resources_are(context, affected_resources):
    # When/Then
    result = assert_is_affected(context)

    # Then
    assert_affected_resources(result, affected_resources)


@then('is not affected.')
def assert_is_not_affected(context):
    # When
    result = execute_is_affected(context)

    # Then
    assert_no_output(result)
    assert_command_unsuccessful(result)
    return result


@then('their is a could not find commit hash "{commit_hash}" error.')
def assert_could_not_find_commit_hash_error(context, commit_hash):
    # Given
    could_not_find_commit_hash_error = f"Can not find a commit with the hash '{commit_hash}'.\n"  # fmt: off

    # When/Then
    result = assert_is_not_affected(context)

    # Then
    assert_error_contains(result, could_not_find_commit_hash_error)


@then(
    'their is a could not find shortened commit hash "{shortened_commit_hash}" error.')
def assert_could_not_find_shortened_commit_hash_error(
        context, shortened_commit_hash):
    # Given
    could_not_find_shortened_commit_hash_error = f"No commit hashes start with the provided short commit hash \"{shortened_commit_hash}\".\n"  # fmt: off

    # When/Then
    result = assert_is_not_affected(context)

    # Then
    assert_error_contains(result, could_not_find_shortened_commit_hash_error)


@then('their is a ambiguous shortened commit hash "{shortened_commit_hash}" error.')
def assert_ambiguous_shortened_commit_hash_error(context, shortened_commit_hash):
    # Given
    ambiguous_shortened_commit_hash_error = re.compile(f"^ ERROR is_affected > Could not find a reference with the name \"{shortened_commit_hash}\".\n\nCaused by:\n    Ambiguous short commit hash, the commit hashes [[]({shortened_commit_hash}[a-f0-9]*(, )?)*[]] all start with the provided short commit hash \"{shortened_commit_hash}\".\n$")  # fmt: off

    # When/Then
    result = assert_is_not_affected(context)

    # Then
    assert_error_matches_regex(result, ambiguous_shortened_commit_hash_error)


@then('their is a could not find reference "{reference}" error.')
def assert_could_not_find_reference_error(context, reference):
    # Given
    could_not_find_reference_error = f"Could not find a reference with the name \"{reference}\".\n"  # fmt: off

    # When/Then
    result = assert_is_not_affected(context)

    # Then
    assert_error_contains(result, could_not_find_reference_error)


@then('their is a missing from argument error.')
def assert_missing_from_argument_error(context):
    # Given
    missing_from_argument_error = "error: the following required arguments were not provided:\n  <FROM>\n\nUsage: is_affected <--affects <AFFECTS>|--affects-current-directory|--list> <FROM>\n\nFor more information, try '--help'.\n"

    # When/Then
    result = assert_is_not_affected(context)

    # Then
    assert_error_equals(result, missing_from_argument_error)


@then('their is a missing output argument error.')
def assert_missing_output_argument_error(context):
    # Given
    missing_output_argument_error = "error: the following required arguments were not provided:\n  <--affects <AFFECTS>|--affects-current-directory|--list>\n\nUsage: is_affected <--affects <AFFECTS>|--affects-current-directory|--list> <FROM>\n\nFor more information, try '--help'.\n"

    # When/Then
    result = assert_is_not_affected(context)

    # Then
    assert_error_equals(result, missing_output_argument_error)


@then('their is a conflicting output arguments error.')
def assert_conflicting_output_arguments_error(context):
    # Given
    conflicting_affects_list_error = "error: the argument '--affects <AFFECTS>' cannot be used with '--list'\n\nUsage: is_affected <--affects <AFFECTS>|--affects-current-directory|--list> <FROM>\n\nFor more information, try '--help'.\n"
    conflicting_affects_affects_current_directory_error = "error: the argument '--affects <AFFECTS>' cannot be used with '--affects-current-directory'\n\nUsage: is_affected <--affects <AFFECTS>|--affects-current-directory|--list> <FROM>\n\nFor more information, try '--help'.\n"
    conflicting_list_affects_error = "error: the argument '--list' cannot be used with '--affects <AFFECTS>'\n\nUsage: is_affected <--affects <AFFECTS>|--affects-current-directory|--list> <FROM>\n\nFor more information, try '--help'.\n"
    conflicting_list_affects_current_directory_error = "error: the argument '--list' cannot be used with '--affects-current-directory'\n\nUsage: is_affected <--affects <AFFECTS>|--affects-current-directory|--list> <FROM>\n\nFor more information, try '--help'.\n"
    conflicting_affects_current_directory_affects_error = "error: the argument '--affects-current-directory' cannot be used with '--affects <AFFECTS>'\n\nUsage: is_affected <--affects <AFFECTS>|--affects-current-directory|--list> <FROM>\n\nFor more information, try '--help'.\n"
    conflicting_affects_current_directory_list_error = "error: the argument '--affects-current-directory' cannot be used with '--list'\n\nUsage: is_affected <--affects <AFFECTS>|--affects-current-directory|--list> <FROM>\n\nFor more information, try '--help'.\n"

    # When/Then
    result = assert_is_not_affected(context)

    # Then
    assert_error_is_one_of(result, [
        conflicting_affects_list_error,
        conflicting_affects_affects_current_directory_error,
        conflicting_list_affects_error,
        conflicting_list_affects_current_directory_error,
        conflicting_affects_current_directory_affects_error,
        conflicting_affects_current_directory_list_error,
    ])
