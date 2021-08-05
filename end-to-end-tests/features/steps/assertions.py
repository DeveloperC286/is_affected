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
