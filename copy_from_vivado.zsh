#!/bin/zsh
cd "$(dirname "$0")" || exit;

# Take the argument of the path of the vivado project
# and the path of the output directory.
project_loc="$1"

## Copy files from the vivado project to the directories here
# Take this repository's directory paths as variables
constraint_dir="constraint_sources"
design_dir="design_sources"
simulation_dir="simulation_sources"
wfcg_dir="waveform_sources"

# Use 'find' to get the path of the *.srcs directory only in the project location
source_loc=$(find "$project_loc" -type d -name "*.srcs")
echo "Source location:"
echo "$source_loc"

if [ -d "$source_loc/constrs_1/new" ] && [ -d "$source_loc/sim_1/new" ] && [ -d "$source_loc/sources_1/new" ]; then
    cp -r "$source_loc/constrs_1/new/$constraint_dir" .
    cp -r "$source_loc/sources_1/new/$design_dir" .
    cp -r "$source_loc/sim_1/new/$simulation_dir" .
    echo "Copied files!"
else
    echo "Error in copying/linking files"
    echo "Please check if the */new* directories are present in the .srcs directory"
    echo "This should not happen, because it will check and create them themselves, but lets check for edge cases"
    exit 1
fi