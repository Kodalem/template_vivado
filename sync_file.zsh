#!/bin/zsh
cd "$(dirname "$0")" || exit;

# Take the argument of the path of the vivado project
# and the path of the output directory.
project_loc="$1"

# Take this repository's directory paths as variables
constraint_dir="constraint_sources"
design_dir="design_sources"
simulation_dir="simulation_sources"
wfcg_dir="waveform_sources"

# Debug path contents of the arguments
ls -l "$project_loc"

# Use 'find' to get the path of the *.srcs directory only in the project location
source_loc=$(find "$project_loc" -type d -name "*.srcs")
# Debug the source location
# echo "$source_loc"

# Use find again to get the path of the .xpr file in the project location
xpr_loc=$(find "$project_loc" -type f -name "*.xpr")

# Check if the project location is valid, but checking if there is a directory with the .srcs directory and the .xpr file
if [ ! -d "$source_loc" ] && [ ! -f "$xpr_loc" ]; then
    echo "Invalid project location"
    exit 1
fi

# Add constraint, design and simulation files to the project, in the .srcs directory, by their source_1, sim_1
# and constrs_1 directories' files respectively in their 'new' directory.
cp -r "$constraint_dir" "$source_loc/constrs_1/new"
cp -r "$design_dir" "$source_loc/sources_1/new"
cp -r "$simulation_dir" "$source_loc/sim_1/new"

# If a waveform file .wcfg is present in the wavefile_config, add it to the base directory of the project as well.
if [ -d "$wfcg_dir" ] || [ -f "$wfcg_dir/*.wcfg" ]; then
    cp -r "$wfcg_dir" "$project_loc"
fi


# Edit the contents of the .xpr file to include the new files by calling the python script
python3 xpr_edit_script.py "$xpr_loc"
# Remove the broken first line of the .xpr file
sed -i '' '1d' "$xpr_loc"
