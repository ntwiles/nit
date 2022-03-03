Nit is a simple, happy-path Git clone, designed as a learning tool to better understand how VCSs (and especially Git) operate under the hood. Contents of the `.nit` directory are not compressed, so that they can be explored and experimented with.

## Commands
 - `init`
    - Initialize the repository by creating a `.nit` folder in the current directory.
 - `add <file name>`
    - Stage the given file.
    - Full (relative) file path must be specified; only a single file can be added at a time. 
 - `status`
    - Print a list of files which have been staged for commit.
    - Untracked files are not yet displayed.
 - `commit`
    - Create a commit from all staged files.
    - Commit messages and other metadata are not yet implemented.
 - `checkout <commit hash>`
    - Populate the working copy from the given commit.
