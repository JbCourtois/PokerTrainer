import os

FOLDERS = []

# Walk through all the subdirectories in the root directory
for root, _, files in os.walk('Spots'):
    # Check if the current directory contains both of the files
    if 'output_parameters.txt' in files and 'output_strategy.json' in files:
        FOLDERS.append(root)

if not FOLDERS:
    FOLDERS = ['./tests/fixtures/sample_hand']
