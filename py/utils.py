def ensure_trailing_slash(path: str):
    if not path.endswith("/"):
        path += "/"
    return path


class Colour:
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

    def red(self, s: str) -> str:
        return self.RED + s + self.END

    def bold(self, s: str) -> str:
        return self.BOLD + s + self.END

    def blue(self, s: str) -> str:
        return self.BLUE + s + self.END


COLOUR = Colour()


def bool_input(prompt: str, default: bool = None) -> bool:
    while True:
        # captialize the default option
        options = (
            "(" + ("Y" if default else "y") + "/" + ("N" if not default else "n") + ")"
        )
        x = input(f"{prompt} {options}\n>> ").lower()
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
