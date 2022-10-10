# HWP-RS
> 본 제품은 한글과컴퓨터의 한/글 문서 파일(.hwp) 공개 문서를 참고하여 개발하였습니다.

Rust로 작성된 hwp파서와 각종 도구들

[hwp-rs와 libhwp를 공개합니다](https://blog.hanlee.io/2022/hwp-rs)


- [hwp-rs](./crates/hwp) Rust로 작성된 로우레벨 hwp 파서
- [libhwp](./crates/python) Rust로 작성된 Python hwp 리더 라이브러리
```python
from libhwp import HWPReader

hwp = HWPReader('<파일 경로>')

# 모든 문단 출력 (표, 캡션 포함)
for paragraph in hwp.find_all('paragraph'):
    print(paragraph)

# 테이블 내용 출력
for table in hwp.find_all('table'):
    for cell in table.cells:
        for paragraph in cell.paragraphs:
            print(paragraph)

# 문서에 사용된 파일 저장
for file in hwp.bin_data:
    with open(file.name, 'wb') as f:
        f.write(file.data)
```

# 개발가이드
[가이드 문서](./docs/development.md)를 참고해주세요

# 다른언어 바인딩은 지원 계획이 있나요?
없습니다.

하지만 [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)을 이용한 웹 어셈블리 바인딩은 준비중이며, [napi-rs](https://napi.rs)를 사용한 node 바인딩은 고려하고 있습니다.

필요시 hwp-rs를 직접 바인딩하여 사용해주세요. 만약 장기적인 관리를 해주실 수 있다면 [discussions](https://github.com/hahnlee/hwp-rs/discussions)에 알려주세요.

# License
```
Copyright Han Lee <hanlee.dev@gmail.com> and other contributors

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
