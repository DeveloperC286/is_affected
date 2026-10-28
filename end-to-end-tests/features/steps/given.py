import hashlib
import os

from assertions import assert_command_successful
from behave import given
from utilities import execute_command


@given('the arguments are reset.')
def reset_arguments(context):
    context.arguments = ""
    context.from_ref = ""


def reset_context(context):
    context.behave_directory = os.getcwd()

    context.is_affected_path = f"{context.behave_directory}/../target/debug/is_affected"
    reset_arguments(context)
    context.environment_overrides = {}


@given('the repository "{remote_repository}" is cloned and checked out at the commit "{commit_hash}".')
def clone_remote_repository_and_checkout_commit(context, remote_repository, commit_hash):
    reset_context(context)

    remote_repository_md5 = hashlib.md5(remote_repository.encode())
    context.remote_repository_cache = f"/tmp/{remote_repository_md5.hexdigest()}/{commit_hash}"

    if not os.path.exists(context.remote_repository_cache):
        result = execute_command(f"git clone {remote_repository} {context.remote_repository_cache}")
        assert_command_successful(result)

        result = execute_command(f"git checkout {commit_hash}", cwd=context.remote_repository_cache)
        assert_command_successful(result)


@given('the GIT_DIR environment variable is set to the cloned repository.')
def set_git_dir(context):
    context.environment_overrides["GIT_DIR"] = str(context.remote_repository_cache + "/.git")


@given('within the directory "{directory}" inside the cloned repository.')
def change_directory(context, directory):
    context.remote_repository_cache += f"/{directory}"
