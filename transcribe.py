import whisper
import sys

input_file = sys.argv[1]

model = whisper.load_model("base")
result = model.transcribe(input_file)
print(result["text"])