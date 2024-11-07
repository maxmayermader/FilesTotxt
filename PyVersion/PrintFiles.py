import os


def combine_code_files(root_dir, output_file):
    # File extensions to look for
    code_extensions = {'.java', '.py', '.cpp', '.h', '.cs', '.js', '.ts'}

    try:
        with open(output_file, 'w', encoding='utf-8') as outfile:
            for root, dirs, files in os.walk(root_dir):
                # Skip .idea directory
                if '.idea' in root:
                    continue

                for file in files:
                    # Check if file has code extension
                    if any(file.endswith(ext) for ext in code_extensions):
                        file_path = os.path.join(root, file)
                        relative_path = os.path.relpath(file_path, root_dir)

                        try:
                            with open(file_path, 'r', encoding='utf-8') as infile:
                                outfile.write(f"\n{'=' * 50}\n")
                                outfile.write(f"File: {relative_path}\n")
                                outfile.write(f"{'=' * 50}\n\n")
                                outfile.write(infile.read())
                                outfile.write("\n\n")
                            print(f"Added: {relative_path}")
                        except Exception as e:
                            print(f"Error reading {file_path}: {str(e)}")

        print(f"\nAll code files have been combined into {output_file}")

    except Exception as e:
        print(f"Error: {str(e)}")


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description='Combine all code files into a single text file')
    parser.add_argument('--src', default='src', help='Source directory (default: src)')
    parser.add_argument('--output', default='combined_code.txt', help='Output file (default: combined_code.txt)')

    args = parser.parse_args()
    combine_code_files(args.src, args.output)