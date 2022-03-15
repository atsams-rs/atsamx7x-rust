#!/usr/bin/env python3
import subprocess, glob, pathlib, shutil, os, string, getopt, sys

def generate_cargo(crate: str, location: pathlib.Path):
    template = ""
    with open("Cargo.toml.template") as templatefile:
        template = templatefile.read()
    with open(location.joinpath("Cargo.toml"), "w") as cargotomlfile:
        cargotomlcontent = string.Template(template).substitute(crate=crate, mcu=crate.upper())
        cargotomlfile.write(cargotomlcontent)

def generate_readme(crate: str, location: pathlib.Path):
    template = ""
    with open("README.md.template") as templatefile:
        template = templatefile.read()
    with open(location.joinpath("README.md"), "w") as cargotomlfile:
        cargotomlcontent = string.Template(template).substitute(crate=crate, mcu=crate.upper())
        cargotomlfile.write(cargotomlcontent)

def throw_help_and_leave():
    print("%s [-f|--force] [-n|--nightly]" % sys.argv[0], file=sys.stderr)
    exit(2)

def cargo_install_command(arguments, nightly = False, force = False):
    return ["cargo"] \
         + [n for n in ["+nightly"] if nightly] \
         + ["install"] \
         + [f for f in ["--force"] if force] \
         + arguments

cargo_force_install = False
use_nightly_features = False
try:
    opts, args = getopt.getopt(sys.argv[1:], "hfn", ["help", "force", "nightly"])
except getopt.GetoptError:
    throw_help_and_leave()
for opt, arg in opts:
    if opt in ("-h", "--help"):
        throw_help_and_leave()
    elif opt in ("-f", "--force"):
        cargo_force_install = True
    elif opt in ("-n", "--nightly"):
        use_nightly_features = True

subprocess.run(cargo_install_command(["svd2rust"], nightly=use_nightly_features, force=cargo_force_install))
subprocess.run(cargo_install_command(["form"], nightly=use_nightly_features, force=cargo_force_install))

all_svd_files = glob.glob("./svd/ATSAM[E,S,V]7[0,1]*.svd")

for svd_file in all_svd_files:
    pac_name = pathlib.Path(svd_file).stem.lower()
    pac_path = pathlib.Path("pac/%s" % pac_name)
    print("Converting {} to {}".format(svd_file, pac_path))
    shutil.rmtree(pac_path.joinpath("src"), ignore_errors=True)
    os.makedirs(pac_path.joinpath("src"))
    generate_cargo(pac_name, pac_path)
    generate_readme(pac_name, pac_path)
    subprocess.run(["svd2rust"] + [n for n in "--nightly" if use_nightly_features] + ["-i", str(pathlib.Path(svd_file).absolute())], cwd=pac_path)
    subprocess.run(["form", "-i", "lib.rs", "-o", "src"], cwd=pac_path)
    os.remove(pac_path.joinpath("lib.rs"))
    subprocess.run(["cargo"] + [n for n in "+nightly" if use_nightly_features] + ["fmt"], cwd=pac_path)
