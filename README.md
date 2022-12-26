# nirust

Nirust is a collection of NeuroImaging (ni ...) command line tools that are useful for
me in my daily research work, implemented in Rust (... rust).

# Set up

To be able to run this program make sure to install [Rust](https://www.rust-lang.org/tools/install) on your system.
After that simply clone this repository and install like so:

```
git clone https://github.com/LeSasse/nirust.git
cd nirust
cargo install --path .
```

# How to use

Nirust consists of a collection of commands, which you can look up by running
`nirust --help`.

```
Usage: nirust <COMMAND>

Commands:
  mask-hemi  Mask the left or right hemisphere of the nifti image
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information  
```

To further look up specific commands, you can run `--help` for each command.
For example, `nirust mask-hemi` results in:

```
Mask the left or right hemisphere of the nifti image

Usage: nirust mask-hemi <INPUT_NIFTI> <OUTPUT_NIFTI> <SIDE>

Arguments:
  <INPUT_NIFTI>   NIfTI file to mask
  <OUTPUT_NIFTI>  Path to store masked NIfTI
  <SIDE>          Mask 'left' or 'right' hemisphere

Options:
  -h, --help  Print help information
```