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
