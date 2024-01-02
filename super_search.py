#!/usr/bin/env python
# ^^ in order to make executable
# 14-12-23


import sys
from os import listdir, getcwd
from os.path import isdir, isfile


class colour:
    PURPLE = "\033[95m"
    CYAN = "\033[96m"
    DARKCYAN = "\033[36m"
    BLUE = "\033[94m"
    GREEN = "\033[92m"
    YELLOW = "\033[93m"
    RED = "\033[91m"
    BOLD = "\033[1m"
    UNDERLINE = "\033[4m"
    END = "\033[0m"

    def bold(self, s: str) -> str:
        return self.BOLD + s + self.END

    def blue(self, s: str) -> str:
        return self.BLUE + s + self.END


def bool_input(prompt: str, default: bool = None) -> bool:
    while True:
        x = input(prompt).lower()
        if x.strip() == "":
            if default is not None:
                return default
            else:
                print("No default provided.")
                continue
        elif x in ["y", "yes"]:
            return True
        elif x in ["n", "no"]:
            return False
        else:
            print("Invalid input.")
            continue

def get_children(path: str) -> ([str], [str]):
    path = ensure_trailing_slash(path)
    try:
        folders = get_subfolders(path)
        files = get_subfiles(path)
        return folders, files
    except FileNotFoundError:
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

def count_descendants(path: str) -> int:
    return 15


def main():
    working_dir = getcwd()
    working_dir = ensure_trailing_slash(working_dir)

    # args = handle_args(sys.argv, [], ["d:int", "s:int", "h:None"])

    if len(sys.argv) > 1:
        working_dir += sys.argv[1]

    if not isdir(working_dir):
        raise NotADirectoryError("The provided path is not a directory")

    file_count_warning_cutoff = 10
    file_count = count_descendants(working_dir)
    if file_count > file_count_warning_cutoff:
        bool_input(
            f"Warning: greater than {file_count_warning_cutoff} files - continue?")

    print_dir(working_dir)


# driver
main()
