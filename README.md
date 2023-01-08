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

# Try it out

## Get some data

If you don't have any neuroimaging data but still want to quickly try out nirust,
you can get the data I use for testing and benchmarking performance and correctness
of the implementations. To get the data and run some tests and the benchmarks,
it is likely best to set up a python virtual environment and install necessary
dependencies in there. Go to the benchmarks directory (`cd benchmarks`) and run:

```sh
python3 -m venv .venv
source .venv/bin/activate
pip install -U pip
```

To install necessary dependencies, run `pip install -r requirements.txt`.
Then go to the dataprep directory (`cd dataprep`) and make sure `get_aomic_piop1_subject.sh`
is executable, then run `./get_aomic_piop1_subject.sh`.

## Example commands

If you got the data as outlined above the following commands should work as they
are, otherwise replace the file paths with whatever data you want to use.

1. Calculate the voxel-wise temporal signal-to-noise ratio of a 4D NIfTI image: 
```zsh
nirust temporal-snr \
  benchmarks/data/sub-0001_task-restingstate_acq-mb3_space-MNI152NLin2009cAsym_desc-preproc_bold.nii \
  temporal_snr_image.nii.gz
```

2. Resample a parcellation to a lower-resolution 4D BOLD NIfTI image:
```zsh
nirust resample-to-image \
  benchmarks/data/parcellations/schaefer_2018/Schaefer2018_100Parcels_7Networks_order_FSLMNI152_1mm.nii.gz \
  benchmarks/data/sub-0001_task-restingstate_acq-mb3_space-MNI152NLin2009cAsym_desc-preproc_bold.nii \
  resampled_parcellation.nii.gz
```

# Affine transformations

Since NIfTI images give information for three different potential affines 
(sform, qform, and fall-back affine), whenever a function (for example resampling)
requires affine transformations, the program will follow [the conventions outlined
by the Python-based nibabel library](https://nipy.org/nibabel/nifti_images.html#choosing-the-image-affine):

> 1. If sform_code != 0 (‘unknown’) use the sform affine; else
> 2. If qform_code != 0 (‘unknown’) use the qform affine; else
> 3. Use the fall-back affine.

In general, nibabel has great documentation and some excellent tutorials [on the
NIfTI file format and affine transformations](https://nipy.org/nibabel/tutorials.html).
You can also refer to the [official NIfTI file specifications for more information](https://nifti.nimh.nih.gov/pub/dist/src/niftilib/nifti1.h).
  