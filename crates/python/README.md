# hwppy
Python binding for hwp-rs

# Example
```python
from hwppy import HWP

hwp = HWP('./path-to-file')

if hwp.header.flags.distributed:
    for section in hwp.view_texts.sections:
        for paragraph in section.paragraphs:
            print(paragraph)
else:
    for section in hwp.body_texts.sections:
        for paragraph in section.paragraphs:
            print(paragraph)
```

## Develop
```
pip install maturin
maturin develop
python
```
