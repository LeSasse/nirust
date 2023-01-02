
import argparse
from nilearn import maskers, image
from ptpython.ipython import embed

def parse_args():
    
    parser = argparse.ArgumentParser(description="Parcellate a nifti image")
    
    parser.add_argument(
        "input_nifti",
        type=str,
        help="nifti image to parcellate"
    )
    parser.add_argument(
        "parcellation_nifti",
        type=str,
        help="parcellation scheme to use"
    )
    parser.add_argument("Output csv file")
    return parser.parse_args()

def main():
    args = parse_args()
    # masker = maskers.NiftiLabelsMasker(args.parcellation_nifti, force_resample=True)
    # result = masker.fit_transform(args.input_nifti)
    #embed()
    image.resample_to_img(
        args.parcellation_nifti,
        args.input_nifti,
        interpolation="nearest",
        #force_resample=True
    )
    
if __name__ == "__main__":
    main()
