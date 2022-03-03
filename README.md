Nit is a simple, happy-path Git clone, designed as a learning tool to better understand how VCSs (and especially Git) operate under the hood. Contents of the `.nit` directory are not compressed, so that they can be explored and experimented with.

## Commands
 - `init`
    - Initialize the repository by creating a `.nit` folder in the current directory.
 - `add <file name>`
    - Stage the given file.
 - `status`
    - Print a list of files which have been staged for commit.
 - `commit`
    - Create a commit using all staged files.
 - `checkout <commit hash>`
    - Populate the working copy from the given commit.
