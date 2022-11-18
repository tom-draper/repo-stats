# Repo Stats

A tool for generating statistical overview for a local repository.

## Installation

```bash
git clone https://github.com/tom-draper/repo-stats.git
cd img-crop
cargo build --release
```

This builds the executable in `/target/release`.

## Usage

To run in the current directory, checking all files, simply run the program without args.
When the program is run without arguments, it will run in the current directory, checking all subfolders recursively.

```bash
repo-stats
```

#### Ignore directory

To ignore a specific directory, use the `-i` or `--ignore` flag followed by the name of the directory to ignore. Any directory path containing this directory name will be ignored.

```bash
repo-stats -i bin
```

To ignore multiple directories, separate the directory names by commas.

```bash
repo-stats -i bin,target,.vscode
```

#### Target directory

To target a specific directory, use the '-t' or '--target' flag followed by the name of the directory to target. Only paths containing this target directory will be used.

```bash
repo-stats -t src
```

#### Remote repository

To run on a remote GitHub repository, specify the repo name with the `-r` or `--remote` flag. The repo name must be specified in the form <user>/<repo>.

```bath
repo-stats -r tom-draper/repo-stats
```
