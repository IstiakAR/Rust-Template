#!/usr/bin/env python3

# python run.py [binary_name]

import subprocess
import os
import sys
import time
import platform
import re
import shutil

RUST_VERSION = "1.85.1"
BUNDLER = "rust_bundler_cp"
STRIP_OUTPUT = "--remove_unused_mod"
BLEEDING = False


def check_temporary_path():
    if not os.path.exists("./result"):
        os.makedirs("./result")
    return "./result"


TEMP_DIRECTORY = check_temporary_path()

BACKUP_DIRECTORY = "backup"
RS_FILE_DIRECTORY = "src/bin/"
TEMPLATE_RS_FILE_NAME = "_template.rs"
CARGO_TOML = "Cargo.toml"


def get_time_str():
    return time.strftime("%H_%M_%S", time.localtime())


BUNDLING_TIME = get_time_str()


def check_rust_toolkit():
    if not shutil.which("rustup"):
        print("rustup not found; skipped rust version enforce")
        return
    subprocess.run(["rustup", "default", RUST_VERSION])


def check_valid_cargo_directory():
    x = os.listdir()
    if CARGO_TOML not in x:
        print("Not a cargo project. Aborting")
        exit(1)


def get_rust_edition() -> str:
    with open(CARGO_TOML, "r", encoding="utf-8") as f:
        cargo = f.read()
    match = re.search(r'(?m)^\s*edition\s*=\s*"(\d{4})"\s*$', cargo)
    if match:
        return match.group(1)
    return "2021"


def bundle(binary) -> str:
    output_path = TEMP_DIRECTORY + "/problem_" + binary + "_" + BUNDLING_TIME
    os.makedirs(TEMP_DIRECTORY, exist_ok=True)
    cmd = [BUNDLER, "--input", ".", "--binary", binary]
    if STRIP_OUTPUT:
        cmd.append(STRIP_OUTPUT)
    cmd.extend(["--output", output_path + ".rs"])

    try:
        subprocess.run(cmd, check=True)
    except subprocess.CalledProcessError:
        if STRIP_OUTPUT:
            print("Bundler retry without --remove_unused_mod")
            fallback = [BUNDLER, "--input", ".", "--binary", binary, "--output", output_path + ".rs"]
            subprocess.run(fallback, check=True)
        else:
            raise

    char_count = os.path.getsize(output_path + ".rs")
    print(f"Bundled source size: {char_count} chars")
    if char_count > 65535:
        print("WARNING: exceeds 65535-char limit")
    return output_path


def compile_rs(rs_file):
    edition = get_rust_edition()
    subprocess.run(["rustc", "--edition", edition, rs_file + ".rs", "-o", rs_file], check=True)


def reset_workspace():
    backup_dir = BACKUP_DIRECTORY + "/" + BUNDLING_TIME + "/"
    subprocess.run(["mkdir", "-p", BACKUP_DIRECTORY])
    subprocess.run(["mkdir", "-p", backup_dir])
    for filename in os.listdir(RS_FILE_DIRECTORY):
        if not filename.endswith("rs"):
            continue
        if filename == TEMPLATE_RS_FILE_NAME:
            continue
        subprocess.run(["mv", RS_FILE_DIRECTORY+filename, backup_dir + filename])
        subprocess.run(["cp", RS_FILE_DIRECTORY + TEMPLATE_RS_FILE_NAME, RS_FILE_DIRECTORY+filename])
    print("Previous result code backed up tp " + backup_dir)
    exit(0)



def main():
    check_rust_toolkit()
    check_valid_cargo_directory()

    binary = "rust_codeforce_template"

    if "--reset" in sys.argv:
        reset_workspace()

    if len(sys.argv) >= 2:
        binary = sys.argv[1]
    rs_file = bundle(binary)
    compile_rs(rs_file)
    subprocess.run([rs_file], check=True)


main()
