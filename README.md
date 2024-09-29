This is template repository for creating  Vivado projects without causing conflicts between different contributors for one project with the git version control.

# How to use

1. Fork this repository
2. Clone the forked repository to your local machine.
3. Create a new branch for your project.
4. Create a new Vivado project.
5. To run the synchronize between made files for each folder by running the zsh script.
   6. From this to project: `zsh sync_file.zsh $PATH_TO_YOUR_PROJECT`
   7. From project to here: `zsh copy_from_vivado.zsh $PATH_TO_YOUR_PROJECT`
6. Commit and push your changes to your forked repository.
7. Sync again for each fetching and pulling from your remote repository.


# Optional
You can symlink instead by using the end argument `--symbolic`, but be warned that 
it assumes that Vivado has access to your project files from wherver you copied this repository to.

# Troubleshooting

#### *Oh no, Vivado still says that files are missing!*
Completely close Vivado and reopen it - no, just reloading the project does not work, because it
apparently caches the that files are missing despite them being there afterwards.

#### *I used `--symbolic` and now Vivado can't find my files!*
If your Vivado application does not have access to your copied repository, then it will not see the files. Try copying the repository to a location where Vivado can access it, or if it still does not work, just hard copy this.


#### *I can't run the script!*
Currently this script is written for zsh - I'll may make it bash later, I am just desperate to have sane version control in
Vivado projects in collaboration with others. Zsh is shell I like to use due to its features and plugins for forgetful me, but I should accomodate for others.
