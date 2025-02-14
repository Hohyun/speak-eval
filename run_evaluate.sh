
#!/bin/bash

# Check if argument is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <category> <publish_idx>"
  exit 1
fi

# Activate python venv
~/torch-venv/bin/activate

cd ~/scripts

# Download audio data from the server.
./prepare_eval_data.sh $1 $2

# Do evaluation using Wisper Base Model.
./speak-eval ~/eval-data/reference/$1.mp3 ~/eval-data/$2.webm

# Deactivat python venv
deactivate

