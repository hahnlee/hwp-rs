# libhwp
rust로 작성된 python hwp 리더 라이브러리

> 본 제품은 한글과컴퓨터의 한/글 문서 파일(.hwp) 공개 문서를 참고하여 개발하였습니다.

[hwp-rs와 hwppy를 공개합니다](https://blog.hanlee/hwp-rs)

# API
## find_all
- hwp
- hwp.sections[i]
- hwp.sections[i].paragraphs[i]
등에서 사용할 수 있습니다.

`find_all('tag', recursive=Boolean)`이며 recursive는 기본적으로 True입니다.
### 지원되는 tag
- paragraph: 문단
- table: 표
- caption: 캡션
- equation: 수식
- footnote: 각주
- endnote: 미주
- header: 머리말
- footer: 꼬리말

# 예
```python
from libhwp import HWPReader

hwp = HWPReader('<파일 경로>')

# 모든 문단 출력 (표, 캡션 포함)
for paragraph in hwp.find_all('paragraph'):
    print(paragraph)

# 표 내용 출력 (표 안의 표 포함)
for table in hwp.find_all('table'):
    for cell in table.cells:
        for paragraph in cell.paragraphs:
            print(paragraph)

# 표 내용 출력 (표 안의 표 무시)
for table in hwp.find_all('table', recursive=False):
    for cell in table.cells:
        for paragraph in cell.paragraphs:
            print(paragraph)

# 표 안의 표 내용 출력 방법 2
for table in hwp.find_all('table'):
    for cell in table.cells:
        for paragraph in cell.paragraphs:
            print(paragraph)

            # paragraph에서도 recursive 하게 찾을 수 있다
            for p in paragraph.find_all('paragraph'):
                print(p)

# 수식 내용 출력
for equation in hwp.find_all('equation'):
    print(equation.script)  # eg. f(x)= logx+sinx

# 문서에 사용된 파일 저장
for file in hwp.bin_data:
    with open(file.name, 'wb') as f:
        f.write(file.data)
```
