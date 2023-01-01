#!/usr/bin/zsh


#############################################################################
# Files to get from subject one fmriprep output
#############################################################################


SUBJECT_DIR=derivatives/fmriprep/sub-0001
T1W_MNI=anat/sub-0001_space-MNI152NLin2009cAsym_desc-preproc_T1w.nii.gz
REST_BOLD_MNI=func/sub-0001_task-restingstate_acq-mb3_space-MNI152NLin2009cAsym_desc-preproc_bold.nii.gz

# and some more constants
INIT_DIR=${PWD}
TARGET_DIR=${PWD}/../data
FILE_BOLD_OUTPUT=${TARGET_DIR}/$(basename "${REST_BOLD_MNI}")
FILE_T1W_OUTPUT=${TARGET_DIR}/$(basename "${REST_BOLD_MNI}")


#############################################################################
#############################################################################


alias logit="echo $(date -u) "

logit ${FILE_BOLD_OUTPUT}

function assert_datalad_exists {
  if ! command -v datalad &> /dev/null
  then
    $(logit) "ERROR Datalad could not be found!"
    exit 
  fi
  logit "INFO Datalad found, proceeding to get data..."
}


if [ ! -f ${FILE_BOLD_OUTPUT} ] && [ ! -f ${FILE_T1W_OUTPUT} ];
then

  logit "INFO Getting one subjects fmriprep output for benchmarking!"
  assert_datalad_exists
  logit "INFO Making temporary directory..."
  TMP=$(mktemp -d)
  cd ${TMP}
  
  logit "INFO Cloning from https://github.com/OpenNeuroDatasets/ds002785"
  datalad clone https://github.com/OpenNeuroDatasets/ds002785
  cd ds002785

  logit "INFO Datalad getting ${SUBJECT_DIR}/${T1W_MNI}"
  datalad get ${SUBJECT_DIR}/${T1W_MNI}

  logit "INFO Datalad getting ${SUBJECT_DIR}/${REST_BOLD_MNI}"
  datalad get ${SUBJECT_DIR}/${REST_BOLD_MNI}

  cp -L ${SUBJECT_DIR}/${T1W_MNI} ${TARGET_DIR}
  cp -L ${SUBJECT_DIR}/${REST_BOLD_MNI} ${TARGET_DIR}
  cd ${INIT_DIR}

else
  logit "INFO Test data already there, skipping datalad..."
fi

logit "INFO Nilearn getting Schaefer parcellations [100, 400]..."
python3 get_schaefer_parcs.py

# Check the exit status of the Python script
if [ $? -ne 0 ]; then
  # Display an error message if the Python script failed
  logit " ERROR The Python script failed with exit code $? Check that nilearn is installed!"
fi

logit "INFO Done. Data in ${TARGET_DIR}"