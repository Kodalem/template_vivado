This is template repository for creating  Vivado projects without causing conflicts between different contributors for one project with the git version control.

# How to use

1. Fork this repository
2. Clone the forked repository to your local machine.
3. Create a new branch for your project.
4. Create a new Vivado project - (**Just in case do not use your old project that was not initialized from this template before**)
5. To run the synchronize between made files for each folder by running the zsh script.
6. From this to project: `zsh sync_file.zsh $PATH_TO_YOUR_PROJECT`
8. This will also run a Rust program that will automatically update all files associated with the project.
9. Be sure to re-open the new project to refresh and associate the newly given files.
7. You can force files to be updated onto the repo from the project to here: `zsh copy_from_vivado.zsh $PATH_TO_YOUR_PROJECT`
6. Commit and push your changes to your forked repository.

# What does it do?

This script prepares and changes your virgin newly created Vivado project with all the required folders that Vivado does not create unless asked. Then the python script will edit your .xpr file to add new files onto the project file if they do not exist. Shortly after it will build and run a Rust script that will real time update both repository files and Vivado project files of the the same name, until manually terminated.

# Optional
You can symlink instead by using the end argument `--symbolic`, but be warned that 
it assumes that Vivado has access to your project files from wherver you copied this repository to.

To opt-out of the Rust program, you can use the argument `--no-rt-update` to the script.

# Troubleshooting

#### *Oh no, Vivado still says that files are missing!*
Completely close Vivado and reopen it - no, just reloading the project does not work, because it
apparently caches the that files are missing despite them being there afterwards.

#### *I used `--symbolic` and now Vivado can't find my files!*
If your Vivado application does not have access to your copied repository, then it will not see the files. Try copying the repository to a location where Vivado can access it, or if it still does not work, just hard copy this.

#### *Can I run this in bash?*

***Yes!***

