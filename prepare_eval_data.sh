#!/bin/bash



# Check if directory argument is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <category> <publish_idx>"
  exit 1
fi

# if the refrence file is not present, get it from the server
if [ ! -f ~/eval-data/refrence/$2.mp3 ]; then
  sshpass -p chunk123* rsync -avh chunkeng@aclass.chunk.kr:~/ap_shop/sp_speaking/songs/1/$1/$1.mp3 ~/eval-data/reference
fi

# if student files is not exist, get them from the server
if [ ! -d ~/eval-data/$2 ]; then
  sshpass -p chunk123* rsync -avh chunkeng@aclass.chunk.kr:~/ap_shop/sp_speaking/save/1/$2 ~/eval-data
fi

# Move to the specified directory
cd ~/eval-data/$2 || exit

# List files and sort by the digit after the underscore
sorted_files=$(ls | sort -t'_' -k2,2n)

# Create a text file with a list of sorted webm files
echo "$sorted_files" | sed 's/^/file /' > filelist.txt

# Merge the sorted files using ffmpeg
ffmpeg -f concat -safe 0 -i filelist.txt -c copy -loglevel error "../student/$1_$2.webm"

# Cleanup
rm -rf ~/eval-data/$2

