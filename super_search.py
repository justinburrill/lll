#!/usr/bin/env python
# ^^ in order to make executable
# 14-12-23


from utils import colour
import sys
from os import listdir, getcwd
from os.path import isdir, isfile


def get_children(path: str) -> ([str], [str]):
    path = ensure_trailing_slash(path)
    try:
        folders = get_subfolders(path)
        files = get_subfiles(path)
        return folders, files
    except:
        print(f"Error reading directory '{path}'")


def get_subfolders(path: str) -> [str]:
    return [x for x in listdir(path) if isdir(path + x)]


def get_subfiles(path: str) -> [str]:
    return [x for x in listdir(path) if isfile(path + x)]


def print_dir(
    parent_path: str, SPACE_COUNT: int = 4, DEPTH_LIMIT: int = 5, level: int = 0
):
    parent_path = ensure_trailing_slash(parent_path)
    subfolders, subfiles = get_children(parent_path)
    spaces = " " * (level * SPACE_COUNT)
    for subfolder in subfolders:
        print(spaces + colour.bold(colour, colour.blue(colour, subfolder)))
        subfolder_path = parent_path + subfolder
        grandchildren = get_children(subfolder_path)
        if level >= DEPTH_LIMIT:
            print(spaces + (SPACE_COUNT * " ") + "<depth limit reached>")
        elif len(grandchildren[0]) + len(grandchildren[1]) > 0:
            print_dir(subfolder_path, SPACE_COUNT, DEPTH_LIMIT, level + 1)
        else:
            print(spaces + (SPACE_COUNT * " ") + "<empty dir>")
    for file in subfiles:
        print(spaces + file)


def ensure_trailing_slash(path: str):
    if not path.endswith("/"):
        path += "/"
    return path


def main():
    working_dir = getcwd()
    working_dir = ensure_trailing_slash(working_dir)

    # args = handle_args(sys.argv, [], ["d:int", "s:int", "h:None"])

    if len(sys.argv) > 1:
        working_dir += sys.argv[1]

    if not isdir(working_dir):
        raise NotADirectoryError("The provided path is not a directory")
    print_dir(working_dir)


# driver
main()
