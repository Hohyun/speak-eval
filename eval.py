import speech_recognition as sr
from difflib import SequenceMatcher
import whisper

# 음성 파일을 텍스트로 변환하는 함수
def speech_to_text_whisper(audio_file):
    try:
        model = whisper.load_model("base")
        result = model.transcribe(audio_file)
        return result["text"]
    except sr.UnknownValueError:
        return None
    except sr.RequestError:
        return None

# 음성 파일을 텍스트로 변환하는 함수
def speech_to_text(audio_file):
    recognizer = sr.Recognizer()
    with sr.AudioFile(audio_file) as source:
        audio = recognizer.record(source)
    try:
        text = recognizer.recognize_google(audio, language="en-US")
        return text
    except sr.UnknownValueError:
        return None
    except sr.RequestError:
        return None

# 두 텍스트의 유사도 계산 함수 (0~1 사이 값)
def calculate_similarity(text1, text2):
    return SequenceMatcher(None, text1, text2).ratio()

# 유사도에 따른 평가
def evaluate_similarity(similarity_score):
    if similarity_score == 1:
        return "완벽하게 일치"
    elif similarity_score >= 0.8:
        return "매우 정확함"
    elif similarity_score >= 0.6:
        return "보통 정도"
    elif similarity_score >= 0.4:
        return "약간 틀림"
    else:
        return "많이 틀림"

# 프로그램 실행 예시
def main(original_audio, student_audio):
    # 원본 텍스트 추출
    original_text = speech_to_text_whisper(original_audio)
    # original_text = speech_to_text(original_audio)
    if original_text is None:
        print("원본 음성 인식에 실패했습니다.")
        return

    # 학생의 텍스트 추출
    student_text = speech_to_text_whisper(student_audio)
    if student_text is None:
        print("학생 음성 인식에 실패했습니다.")
        return

    # 유사도 계산
    similarity_score = calculate_similarity(original_text, student_text)

    # 평가 출력
    evaluation = evaluate_similarity(similarity_score)
    print(f"학생의 발음 유사도 평가: {evaluation} (유사도: {similarity_score:.2f})")

# 원본 음성 파일과 학생 음성 파일 경로 설정
original_audio_file = "best1.wav"  # 원본 음성 파일 경로
student_audio_file = "best1.wav"  # 학생 음성 파일 경로

main(original_audio_file, student_audio_file)
