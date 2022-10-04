# HWP-RS
> 본 제품은 한글과컴퓨터의 한/글 문서 파일(.hwp) 공개 문서를 참고하여 개발하였습니다.

로우 레벨 hwp 파서. 아래아 한글의 구조와 최대한 가까운 구조로 파싱합니다.
hwp 파일의 구조를 잘 알고 있지 않다면 사용하기 어려울 수 있습니다.

# Creates
hwp-rs는 다음과 같은 crates로 되어 있습니다.

## hwp
러스트로 작성된 hwp 파서.

### 참고
- 잠정적인 주요 목표는 wasm을 활용한 웹 브라우저 구동입니다.
- 따라서 시스템콜등 브라우저에서 실행 할 수 없는 기능은 거절될 수 있습니다.

## macro
한/글 내부에서 사용하는 `MAKE_4CHID` 함수의 반환값을 패턴매칭에 사용할 수 있도록 만든 매크로.

## python
파이썬 바인딩.

### 참고
- 파이썬 지원은 *주* 목표가 아닙니다.
- 파이썬 버전은 모든 기능을 구현하지 않습니다.
- 파이썬 버전의 소스코드는 main 브랜치여도 릴리즈 전까지 빌드 되지 않을 수 있습니다.

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
