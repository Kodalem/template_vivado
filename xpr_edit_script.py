
import sys
import os
def edit_xpr_script(xpr_path):
    # Prepare the design sources
    const_files, dsrc_files, sim_files = prepare_design_sources()

    # Open the script
    with open(xpr_path, 'r+') as f:
        script = f.read()

        # Print the version
        # print(script)

        # Find location of the string where it contains <Filter Type="Srcs"/>
        src_file_loc = script.find('<Filter Type="Srcs"/>')
        # Add the amount of characters to get to the end of the line
        src_file_loc += len('<Filter Type="Srcs"/>')

        # Then append the new file by following this format of each file in the array
        #<File Path="$PSRCDIR/sources_1/new/PATH_OF_THE_SOURCE_FILE.vhd">
        #    <FileInfo>
        #        <Attr Name="AutoDisabled" Val="1"/>
        #        <Attr Name="UsedIn" Val="synthesis"/>
        #        <Attr Name="UsedIn" Val="simulation"/>
        #    </FileInfo>
        #</File>
        for file in dsrc_files:
            file_path = file.replace('\\', '/')
            # Check if the file exists in the script already
            if script.find(file_path) != -1:
                print(f"File {file_path} already exists in the script")
                continue

            # Write the new file to the .xpr per src file location
            script = script[:src_file_loc] + (f'\n<File Path="$PSRCDIR/sources_1/new/{file_path}">\n   '
                                              f'<FileInfo>\n       '
                                              f'<Attr Name="AutoDisabled" Val="1"/>\n        '
                                              f'<Attr Name="UsedIn" Val="synthesis"/>\n        '
                                              f'<Attr Name="UsedIn" Val="simulation"/>\n    '
                                              f'</FileInfo>\n'
                                              f'<!-- This file was added by Python script -->\n'
                                              f'</File> \n') + script[src_file_loc + 1:]

        # Do the same for the constraint files
        const_file_loc = script.find('<Filter Type="Constrs"/>')
        const_file_loc += len('<Filter Type="Constrs"/>')

        for file in const_files:
            file_path = file.replace('\\', '/')
            # Check if the file exists in the script already
            if script.find(file_path) != -1:
                print(f"File {file_path} already exists in the script")
                continue

            script = script[:const_file_loc] + (f'\n<File Path="$PSRCDIR/constrs_1/new/{file_path}">\n   '
                                              f'<FileInfo>\n       '
                                              f'<Attr Name="AutoDisabled" Val="1"/>\n        '
                                              f'<Attr Name="UsedIn" Val="synthesis"/>\n        '
                                              f'<Attr Name="UsedIn" Val="implementation"/>\n    '
                                              f'</FileInfo>\n'
                                              f'<!-- This file was added by Python script -->\n'
                                              f'</File> \n') + script[const_file_loc + 1:]

        # Do the same for the simulation files
        sim_file_loc = script.find('<FileSet Name="sim_1" Type="SimulationSrcs" RelSrcDir="$PSRCDIR/sim_1" RelGenDir="$PGENDIR/sim_1">')
        sim_file_loc += len('<FileSet Name="sim_1" Type="SimulationSrcs" RelSrcDir="$PSRCDIR/sim_1" RelGenDir="$PGENDIR/sim_1">')

        for file in sim_files:
            file_path = file.replace('\\', '/')
            # Check if the file exists in the script already
            if script.find(file_path) != -1:
                print(f"File {file_path} already exists in the script")
                continue

            script = script[:sim_file_loc] + (f'\n<File Path="$PSRCDIR/sim_1/new/{file_path}">\n   '
                                              f'<FileInfo>\n       '
                                              f'<Attr Name="AutoDisabled" Val="1"/>\n        '
                                              f'<Attr Name="UsedIn" Val="simulation"/>\n        '
                                              f'<Attr Name="UsedIn" Val="synthesis"/>\n    '
                                              f'</FileInfo>\n'
                                              f'<!-- This file was added by Python script -->\n'
                                              f'</File> \n') + script[sim_file_loc + 1:]

        # Delete the old content of the file
        f.truncate(1)
        f.write(script)

def prepare_design_sources():
    # Get the files from the constraint_sources folder
    const_files = files_from_path_to_be_added("constraint_sources")

    # Get the files from the design_sources folder
    dsrc_files = files_from_path_to_be_added("design_sources")

    # Get the files from the simulation_sources folder
    sim_files = files_from_path_to_be_added("simulation_sources")

    return const_files, dsrc_files, sim_files

def files_from_path_to_be_added(path):
    files = []
    for root, dirs, filenames in os.walk(path):
        for filename in filenames:
            files.append(os.path.join(root, filename))
    return files


# Main function
# First argument is the location of the script to edit
xpr_path_arg = sys.argv[1]
print(xpr_path_arg)
edit_xpr_script(xpr_path_arg)


# If ran just from this script, just test the functions
#if __name__ == '__main__':
#    # Check if ran from the JetBrains IDE
#    if any ("jetbrains" in s for s in os.environ):
#        c_files, d_files, s_files = prepare_design_sources()
#        print(c_files)
#        print(d_files)
#        print(s_files)
#        edit_xpr_script("debug_xpr.xml")

