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
A collection of command line utilities for neuroimaging

Usage: nirust [OPTIONS] <COMMAND>

Commands:
  mask-hemi          Mask the left or right hemisphere of a NIfTI image
  temporal-snr       Compute the voxel-wise temporal SNR of a 4D NIfTI image
  parcellate         Parcellate a 3D or 4D NIfTI image
  resample-to-image  Resample a 3D NIfTI image to another 3D or 4D reference image using nearest neighbour interpolation
  help               Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose  Whether to run verbose
  -h, --help     Print help information
  -V, --version  Print version information
```

To further look up specific commands, you can run `--help` for each command.
For example, `nirust mask-hemi --help` results in:

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