# 개발 가이드
## 설치
Rust edition 2021을 사용중입니다. [Rust 공식문서](https://www.rust-lang.org/tools/install)를 참고하여 러스트를 설치해주세요.

## Python 개발 환경
hwppy는 바인딩을 위해 [pyo3](https://github.com/PyO3/pyo3)를 사용합니다.

개발 환경을 [maturin](https://github.com/PyO3/maturin)을 설치해주세요

## 빌드 & 테스트
```
cargo build
cargo test
```

### Python 빌드
```
cd crates/python
maturin develop
```
이후 파이썬 인터프리터에서 `import hwppy`처럼 사용할 수 있습니다.

## 기능 추가
새로운 기능이 필요하다면 아래 사항을 참고해주세요

### (hwp-rs) OS 의존적인 기능은 피해주세요
hwp-rs의 잠정적 목표는 wasm 지원입니다. OS 의존적인 기능은 가급적 피해주세요.

python(hwppy)의 경우에는 괜찮습니다.

### 기능 추가 전 Issue 또는 Discussion을 만들어주세요.
hwp-rs는 가능한 적은 기능을 유지하려고 합니다.

만약 필요한 기능이 있다면 Issue 또는 Discussion을 만들어주세요.
논의 후 추가 할 수 있습니다.

# PR
[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) 가이드에 맞추어 PR제목을 만들어주세요.

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
