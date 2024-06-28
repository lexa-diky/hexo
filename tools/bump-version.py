import os
from typing import Iterable


def list_cargo_files() -> Iterable[str]:
    for [path, _, files] in os.walk(".", topdown=False):
        for file in filter(lambda f: f == "Cargo.toml", files):
            cargo_file_path = os.path.join(path, file)
            yield cargo_file_path


def read_current_version() -> str:
    with open("Cargo.toml", "r") as f:
        for line in f.readlines():
            if line.startswith("version"):
                return line.split("=")[1].strip().strip("\"")


def update_version(old_version: str, new_version: str):
    for cargo_file in list_cargo_files():
        with open(cargo_file, "r") as f:
            lines = f.readlines()

        with open(cargo_file, "w") as f:
            for line in lines:
                if line.startswith("version"):
                    f.write(f'version = "{new_version}"\n')
                else:
                    f.write(line)

current_version = read_current_version()

print("bumping version...")
print(f"current version: {current_version}")
new_version = input("new version: ")
update_version(current_version, new_version)

