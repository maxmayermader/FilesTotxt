# Code Combiner
A Python utility script that combines multiple source code files into a single text file.

## Features
Recursively scans directories for code files
Supports multiple programming languages (.java, .py, .cpp, .h, .cs, .js, .ts)
Maintains file structure information
Adds clear separators between files
UTF-8 encoding support
Skips .idea directories
## Usage
`bash`

`python combine_code.py --src source_directory --output combined.txt `

## Arguments
--src: Source directory to scan (default: 'src')
--output: Output file name (default: 'combined_code.txt')

## Output Format
Each file is separated by '=' lines
File paths are preserved and displayed
Original code formatting is maintained
