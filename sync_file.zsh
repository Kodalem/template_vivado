#!/bin/zsh
cd "$(dirname "$0")" || exit;

# Take the argument of the path of the vivado project
# and the path of the output directory.
project_loc="$1"

# Check the symbolic link --symbolic to specify to use the symbolic link instead
sym_flag="$2"
echo "$sym_flag"
sym_flag_name="--symbolic"

# Tell the user if they are in symbolic mode or hard copy mode
if [[ "$sym_flag" == "$sym_flag_name" ]]; then
    echo "Symbolic link mode"
else
    echo "Hard copy mode"
fi

# Take this repository's directory paths as variables
constraint_dir="constraint_sources"
design_dir="design_sources"
simulation_dir="simulation_sources"
wfcg_dir="waveform_sources"

# Debug path contents of the arguments
# ls -l "$project_loc"

# Use 'find' to get the path of the *.srcs directory only in the project location
source_loc=$(find "$project_loc" -type d -name "*.srcs")
# Debug the source location
# echo "Source location:"
# echo "$source_loc"

# If source location is empty, then create the .srcs directory, by the name of
# the project name in the project location
if [ -z "$source_loc" ]; then
    project_name=$(basename "$project_loc")
    mkdir "$project_loc/$project_name.srcs"
    source_loc="$project_loc/$project_name.srcs"
fi

    # Create the source_1, sim_1 and constrs_1 directories in the .srcs directory,
    # if they do not exist.
if [ ! -d "$source_loc/sources_1" ]; then
    mkdir "$source_loc/sources_1"
    mkdir "$source_loc/sources_1/new"
fi
if [ ! -d "$source_loc/sim_1" ]; then
    mkdir "$source_loc/sim_1"
    mkdir "$source_loc/sim_1/new"
fi
if [ ! -d "$source_loc/constrs_1" ]; then
    mkdir "$source_loc/constrs_1"
    mkdir "$source_loc/constrs_1/new"
fi

# Check if the 'new' are there, if not create them
if [ ! -d "$source_loc/sources_1/new" ]; then
    mkdir "$source_loc/sources_1/new"
fi
if [ ! -d "$source_loc/sim_1/new" ]; then
    mkdir "$source_loc/sim_1/new"
fi
if [ ! -d "$source_loc/constrs_1/new" ]; then
    mkdir "$source_loc/constrs_1/new"
fi

# Use find again to get the path of the .xpr file in the project location
xpr_loc=$(find "$project_loc" -type f -name "*.xpr")

# Check if the project location is valid, but checking if there is a directory with the .srcs directory and the .xpr file
if [ ! -d "$source_loc" ] && [ ! -f "$xpr_loc" ]; then
    echo "Invalid project location"
    exit 1
fi


# Add constraint, design and simulation files to the project, in the .srcs directory, by their source_1, sim_1
# and constrs_1 directories' files respectively in their 'new' directory.
# If the /new/ directory does not exist, create it. Check also if it is on symlink mode.
if [ -d "$source_loc/constrs_1/new" ] && [ -d "$source_loc/sim_1/new" ] && [ -d "$source_loc/sources_1/new" ] && [ -z "$sym_flag" ]; then
    cp -r "$constraint_dir" "$source_loc/constrs_1/new"
    cp -r "$design_dir" "$source_loc/sources_1/new"
    cp -r "$simulation_dir" "$source_loc/sim_1/new"
    echo "Copied files!"
elif [ -d "$source_loc/constrs_1/new" ] && [ -d "$source_loc/sim_1/new" ] && [ -d "$source_loc/sources_1/new" ] && [[ "$sym_flag" == "$sym_flag_name" ]]; then
    ln -s "$constraint_dir" "$source_loc/constrs_1/new"
    ln -s "$design_dir" "$source_loc/sources_1/new"
    ln -s "$simulation_dir" "$source_loc/sim_1/new"
    echo "Linked files!"
else
    echo "Error in copying/linking files"
    echo "Please check if the */new* directories are present in the .srcs directory"
    echo "This should not happen, because it will check and create them themselves, but lets check for edge cases"
    exit 1
fi



# If a waveform file .wcfg is present in the wavefile_config, add it to the base directory of the project as well.
if [ -d "$wfcg_dir" ] || [ -f "$wfcg_dir/*.wcfg" ]; then
    cp -r "$wfcg_dir" "$project_loc"
fi

# Create the backup of the .xpr file, I might have GLWT licence, but I am not an asshole.
# If another back up exists, don't overwrite it, just create a new one, till about 4 backups.
if [ ! -f "$xpr_loc.bak" ]; then
    cp "$xpr_loc" "$xpr_loc.bak"
elif [ ! -f "$xpr_loc.bak1" ]; then
    cp "$xpr_loc" "$xpr_loc.bak1"
elif [ ! -f "$xpr_loc.bak2" ]; then
    cp "$xpr_loc" "$xpr_loc.bak2"
elif [ ! -f "$xpr_loc.bak3" ]; then
    cp "$xpr_loc" "$xpr_loc.bak3"
elif [ ! -f "$xpr_loc.bak4" ]; then
    cp "$xpr_loc" "$xpr_loc.bak4"
else
    # Delete the oldest backup and create a new one
    rm "$xpr_loc.bak"
    cp "$xpr_loc" "$xpr_loc.bak"
fi

# I know that this backup system is very... basic and after 5th it just overwrites the oldest one,
# but alas, I need to actually do the stuff I am supposed to do.

# Edit the contents of the .xpr file to include the new files by calling the python script
python3 xpr_edit_script.py "$xpr_loc"
# Remove the broken first line of the .xpr file
sed -i '' '1d' "$xpr_loc"
