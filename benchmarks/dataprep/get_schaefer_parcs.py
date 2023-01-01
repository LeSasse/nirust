#!/usr/bin/env python3


import os
from nilearn import datasets


def main():
        par_dir = os.path.join("..", "data", "parcellations")
        if not os.path.isdir(par_dir):
            os.mkdir(par_dir)
        
        for n_rois in [100, 400]:
            datasets.fetch_atlas_schaefer_2018(n_rois=n_rois, data_dir=par_dir)


if __name__ == "__main__":
    main()