# Repo Stats

A tool for generating statistical overview for a local repository including metrics like line count, average line length and proportion of whitespace, broken down by language.

## Installation

```bash
git clone https://github.com/tom-draper/repo-stats.git
cd repo-stats
cargo build --release
```

This builds the executable in `/target/release`.

Setting the path to `repo-stats.exe` as an environment variable will allow it to be run in the terminal from within any directory.

## Usage

When the program is run without arguments, the process will be started in the current directory, checking all subfolders recursively and logging all files.

```bash
repo-stats
```

#### Ignore directory

A specific directory can be ignored with the `-i` or `--ignore` flag followed by the name of the directory to ignore. Any directory path containing this directory name will be ignored.

```bash
repo-stats -i bin
```

To ignore multiple directories, separate the directory names by commas.

```bash
repo-stats -i bin,target,.vscode
```

#### Target directory

A specific directory can be targeted using the '-t' or '--target' flag followed by the name of the directory to target. Only paths containing this target directory will be used.

```bash
repo-stats -t src
```

#### Remote repository

Project statistics can also be displayed for a remote repository. Simply specify the name of the remote GitHub repo with the `-r` or `--remote` flag. The repo name must be specified in the form <user>/<repo> and the repo must be public.

```bath
repo-stats -r tom-draper/repo-stats
```

## Example

```text
> repo-stats

---- Total -------------------------
 Files:                       2,386
 Lines:                      34,116
 Characters:              1,880,326
 Average line length:          55.1
 Whitespace:                 10.56%
 Memory:                   572.5 MB
 Carridge returns:             2474
------------------------------------

---- Source code -------------------
 Languages:
 .h: 68.72%
     Files:                      98
     Lines:                  28,004
     Characters:            942,236
     Average line length:      33.6
     Whitespace:             16.53%
     Memory:               942.2 KB
     Carridge returns:           98
 .d: 22.65%
     Files:                     118
     Lines:                   1,553
     Characters:            310,586
     Average line length:     200.0
     Whitespace:              0.69%
     Memory:               310.6 KB
     Carridge returns:          118
 .json: 4.69%
     Files:                     131
     Lines:                     135
     Characters:             64,309
     Average line length:     476.4
     Whitespace:              0.28%
     Memory:                64.3 KB
     Carridge returns:          135
 .rs: 2.94%
     Files:                       4
     Lines:                   1,156
     Characters:             40,257
     Average line length:      34.8
     Whitespace:             41.01%
     Memory:                40.3 KB
     Carridge returns:         1156
 .lock: 0.88%
     Files:                      42
     Lines:                     511
     Characters:             12,035
     Average line length:      23.6
     Whitespace:              4.87%
     Memory:                12.0 KB
     Carridge returns:           42
 .md: 0.11%
     Files:                       1
     Lines:                      53
     Characters:              1,448
     Average line length:      27.3
     Whitespace:             12.15%
     Memory:                 1.4 KB
     Carridge returns:           53
 .toml: 0.02%
     Files:                       1
     Lines:                      15
     Characters:                324
     Average line length:      21.6
     Whitespace:             10.19%
     Memory:                  324 B
     Carridge returns:            1
------------------------------------

---- Binaries ----------------------
 Files: 28
 Memory: 1.0 KB
------------------------------------
```
