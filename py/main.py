#!/usr/bin/env python
# ^^ in order to make executable
# 14-12-23


import sys
from os import getcwd
from os.path import isdir
from utils import *
from files import *


def print_dir(
    parent_path: str, SPACE_COUNT: int = 4, DEPTH_LIMIT: int = 5, level: int = 0
):
    parent_path = ensure_trailing_slash(parent_path)
    subfolders, subfiles = get_children(parent_path)
    spaces = " " * (level * SPACE_COUNT)
    for subfolder in subfolders:
        print(spaces + COLOUR.bold(COLOUR.blue(subfolder)))
        subfolder_path = parent_path + subfolder
        grandchildren = get_children(subfolder_path)
        # check to see how far down we are
        if level >= DEPTH_LIMIT:
            print(spaces + (SPACE_COUNT * " ") + "<depth limit reached>")
        elif len(grandchildren[0]) + len(grandchildren[1]) > 0:
            # catch possible permission error when reading directory
            try:
                print_dir(subfolder_path, SPACE_COUNT, DEPTH_LIMIT, level + 1)
            except PermissionError:
                print(spaces + (SPACE_COUNT * " ") + COLOUR.red("<permission error>"))
        else:
            print(spaces + (SPACE_COUNT * " ") + "<empty dir>")
    for file in subfiles:
        print(spaces + file)


def main():
    FILE_COUNT_WARNING = 100
    DEPTH_LIMIT = 5

    working_dir = getcwd()
    working_dir = ensure_trailing_slash(working_dir)

    # args = handle_args(sys.argv, [], ["d:int", "s:int", "h:None"])

    if len(sys.argv) > 1:
        working_dir += sys.argv[1]

    if not isdir(working_dir):
        raise NotADirectoryError("The provided path is not a directory")

    file_count = count_descendants(working_dir, FILE_COUNT_WARNING)
    if file_count > FILE_COUNT_WARNING:
        if not bool_input(
            COLOUR.bold(
                f"Warning: greater than {FILE_COUNT_WARNING} files - continue?"
            ),
            True,
        ):
            return

    print_dir(working_dir, DEPTH_LIMIT=DEPTH_LIMIT)


if __name__ == "__main__":
    main()
