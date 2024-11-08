from utils import *
from os import listdir
from os.path import isfile, isdir


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


def count_descendants(path: str, warning: int) -> int:
    total_descendants = 0
    (subfolders, subfiles) = get_children(path)
    total_descendants += len(subfolders) + len(subfiles)
    for sf in subfolders:
        if total_descendants > warning:
            return total_descendants
        try:
            total_descendants += count_descendants(
                ensure_trailing_slash(path + sf), warning
            )
        except PermissionError:
            pass
    return total_descendants
