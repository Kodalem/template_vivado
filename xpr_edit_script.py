import json
import xmltodict
import sys
import os


def convert_xpr_to_json(xpr_path):
    # Open the script
    with open(xpr_path, 'r') as f:
        script = f.read()

        # Convert the script to JSON
        json_dump = json.dumps(xmltodict.parse(script))
        return json_dump


def convert_xpr_to_dict(xpr_path):
    # Open the script
    with open(xpr_path, 'r') as f:
        script = f.read()

        # Convert the script to JSON
        dict_dump = xmltodict.parse(script)
        return dict_dump


def edit_xpr_script(xpr_path):
    # Prepare the design sources
    const_files, dsrc_files, sim_files = prepare_design_sources()

    # Convert the xpr file to a dictionary
    dict_dump = convert_xpr_to_dict(xpr_path_arg)
    print(dict_dump)

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

            # Seek file name
            file_name = file_path.split('/')[-1]
            print(file_name)

            #print(dict_dump['Project']['FileSets']['FileSet'][0]['File'] )
            dict_dump['Project']['FileSets']['FileSet'][0]['File'].append(file_builder("design_sources", file_name))
            #print(dict_dump['Project']['FileSets']['FileSet'][0]['File'])

        # Do the same for the constraint files
        const_file_loc = script.find('<Filter Type="Constrs"/>')
        const_file_loc += len('<Filter Type="Constrs"/>')

        for file in const_files:
            file_path = file.replace('\\', '/')
            # Check if the file exists in the script already
            if script.find(file_path) != -1:
                print(f"File {file_path} already exists in the script")
                continue

            # Seek file name
            file_name = file_path.split('/')[-1]
            print(file_name)
            print(dict_dump['Project']['FileSets']['FileSet'][1]['File'])
            dict_dump['Project']['FileSets']['FileSet'][1]['File'].update(file_builder("constraint_sources", file_name))
            print(dict_dump['Project']['FileSets']['FileSet'][1]['File'])

        # Do the same for the simulation files
        sim_file_loc = script.find(
            '<FileSet Name="sim_1" Type="SimulationSrcs" RelSrcDir="$PSRCDIR/sim_1" RelGenDir="$PGENDIR/sim_1">')
        sim_file_loc += len(
            '<FileSet Name="sim_1" Type="SimulationSrcs" RelSrcDir="$PSRCDIR/sim_1" RelGenDir="$PGENDIR/sim_1">')

        for file in sim_files:
            file_path = file.replace('\\', '/')
            # Check if the file exists in the script already
            if script.find(file_path) != -1:
                print(f"File {file_path} already exists in the script")
                continue

            # Seek file name
            file_name = file_path.split('/')[-1]
            #print(file_name)

            dict_dump['Project']['FileSets']['FileSet'][2]['File'].append(file_builder("simulation_sources", file_name))
            #print(dict_dump['Project']['FileSets']['FileSet'][2]['File'])

        # Replace the script with the new one
        f.seek(0)
        f.write(xmltodict.unparse(dict_dump, pretty=True))
        f.truncate()



def file_builder(source_type, file_name):
    file_path = '$PSRCDIR/sources_1/new/' + source_type + '/' + file_name
    match source_type:
        case "design_sources":
            dictionary = {
                '@Path': file_path,
                'FileInfo': {
                    'Attr': [
                        {
                            '@Name': 'AutoDisabled',
                            '@Val': '1'
                        },
                        {
                            '@Name': 'UsedIn',
                            '@Val': 'synthesis'
                        },
                        {
                            '@Name': 'UsedIn',
                            '@Val': 'simulation'
                        }
                    ]
                }
            }
        case "constraint_sources":
            dictionary = {
                '@Path': file_path,
                'FileInfo': {
                    'Attr': [
                        {
                            '@Name': 'UsedIn',
                            '@Val': 'synthesis'
                        },
                        {
                            '@Name': 'UsedIn',
                            '@Val': 'implementation'
                        }
                    ]
                }
            }
        case "simulation_sources":
            dictionary = {
                '@Path': file_path,
                'FileInfo': {
                    'Attr': [
                        {
                            '@Name': 'AutoDisabled',
                            '@Val': '1'
                        },
                        {
                            '@Name': 'UsedIn',
                            '@Val': 'simulation'
                        },
                        {
                            '@Name': 'UsedIn',
                            '@Val': 'synthesis'
                        }
                    ]
                }
            }
    return dictionary


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
#dict_dump = convert_xpr_to_dict(xpr_path_arg)


#print(dict_dump)


# Read how many files they are in the FileSet
def read_fileset_amount(dict_dump):
    for i in range(len(dict_dump['Project']['FileSets']['FileSet'])):
        j = 0
        print(dict_dump['Project']['FileSets']['FileSet'][i]['@Name'])
        print(len(dict_dump['Project']['FileSets']['FileSet'][i]['File']))
        for j in range(len(dict_dump['Project']['FileSets']['FileSet'][i]['File'])):
            # Key error exception handling
            while True:
                try:
                    print(dict_dump['Project']['FileSets']['FileSet'][i]['File'][j]['@Path'])
                    break
                except KeyError:
                    print(dict_dump['Project']['FileSets']['FileSet'][i]['File']['@Path'])
                    break


def add_file_to_fileset(dict_dump, file_name, source_type):
    for i in range(len(dict_dump['Project']['FileSets']['FileSet'])):
        # source_file_amount = len(dict_dump['Project']['FileSets']['FileSet'][i]['File'])
        # print(source_file_amount)
        # print(dict_dump['Project']['FileSets']['FileSet'][i]['File'] )
        dict_dump['Project']['FileSets']['FileSet'][i]['File'].append(file_builder(source_type, file_name))
        print(dict_dump['Project']['FileSets']['FileSet'][i]['File'])


### TODO ACCOMODATE UTILS FILESET


# If ran just from this script, just test the functions
#if __name__ == '__main__':
#    # Check if ran from the JetBrains IDE
#    if any ("jetbrains" in s for s in os.environ):
#        c_files, d_files, s_files = prepare_design_sources()
#        print(c_files)
#        print(d_files)
#        print(s_files)
#        edit_xpr_script("debug_xpr.xml")
