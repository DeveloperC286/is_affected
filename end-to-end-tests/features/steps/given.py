import os
import tempfile
from behave import given

from util import execute_command


def reset_arguments(context):
    context.arguments = ""


def reset_context(context):
    context.behave_directory = os.getcwd()
    context.temporary_directory = tempfile.TemporaryDirectory()

    context.is_effected_path = context.behave_directory + \
        "/../target/debug/is_effected"
    reset_arguments(context)

    if "GIT_DIR" in os.environ:
        del os.environ["GIT_DIR"]


@given('the repository "{remote_repository}" is cloned and checked out at the commit "{commit_hash}".')
def clone_remote_repository_and_checkout_commit(
        context, remote_repository, commit_hash):
    reset_context(context)

    os.chdir(context.temporary_directory.name)
    (exit_code, _, _) = execute_command(
        "git clone " + remote_repository + " .")
    assert exit_code == 0
    (exit_code, _, _) = execute_command("git checkout " + commit_hash)
    assert exit_code == 0

    os.chdir(context.behave_directory)