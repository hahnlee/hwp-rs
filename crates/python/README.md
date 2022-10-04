# hwppy
Python binding for hwp-rs

# Example
```python
from hwppy import HWP

hwp = HWP('./path-to-file')

if hwp.header.flags.distributed:
    # 배포용 문서면 view_texts를 읽어야합니다.
    # 참고: 배포용 문서도 하위버전을 위해 body_texts가 있는 경우도 있습니다.
    for section in hwp.view_texts.sections:
        for paragraph in section.paragraphs:
            # 참고: 표, 자동번호, 글쓰기, 수식등은 무시됩니다.
            # 위 정보가 필요하다면 직접 visitor를 구현하셔야 합니다.
            print(paragraph)
else:
    # 일반 문서는 body_texts만 존재합니다
    for section in hwp.body_texts.sections:
        for paragraph in section.paragraphs:
            print(paragraph)
```

## paragraph.to_string 직접 구현하기
```python
def paragraph_to_string(chars):
    result = []

    for char in chars:
        if char.kind == 'char_code':
            result.append(chr(char.code))
        if char.kind == 'char_control' and char.code == 10:
            result.append('\n')
        # 표 안의 내용을 읽고 싶다면 추가로 수정해야한다.
    return ''.join(result)

for section in hwp.body_texts.sections:
    for paragraph in section.paragraphs:
        print(paragraph_to_string(paragraph.chars()))
```

# Develop
```
pip install maturin
maturin develop
python
```
