---
description: 이번 주 단어로 업무 문장을 영작하고 주간 통계를 낸다
argument-hint: [첨삭받을 영어 문장들(PR 제목, 커밋 메시지 등) — 선택]
---

추가 입력: $ARGUMENTS

## 1. 주간 통계

`vocab.md`, `grammar.md`, `articles/`를 읽고 다음을 계산해 보여준다.

- 이번 주(월요일~오늘) 읽은 문서 수, 추가한 단어·문법 수
- 상자별 항목 수 (1~5, ✓)
- 이번 주 졸업한 항목 (git log로 확인할 수 있으면 사용하고, 없으면 생략)
- 약점 항목: 오답 3회 이상인 단어·문법 목록

## 2. 영작 연습

- 이번 주 추가한 단어 중 5~10개를 고른다. 이번 주 추가분이 5개보다 적으면 약점 항목으로 채운다.
- 단어마다 **업무 상황**을 한 줄 제시하고 (커밋 메시지, PR 제목, 코드 리뷰 코멘트, 이슈 설명, 팀 채팅 등), 그 단어를 써서 영어 문장을 쓰게 한다.
- 한 번에 모든 상황을 보여주고 사용자가 한꺼번에 답하게 한다.

## 3. 첨삭

사용자의 문장마다 다음 형식으로 첨삭한다.

```
**1. mitigate** — 상황: 성능 저하를 완화하는 PR 제목
- 내가 쓴 문장: Mitigate performance degrade in scheduler.
- 교정: Mitigate performance degradation in the scheduler.
- 이유: degrade는 동사, 명사형은 degradation. 특정 모듈이므로 the를 붙인다.
- 더 자연스러운 표현: Reduce scheduler slowdown under heavy load.
```

추가 입력으로 실제 PR 제목이나 커밋 메시지를 받았다면 같은 형식으로 함께 첨삭한다. PR 제목은 영어 한 문장이라는 팀 규칙을 기준으로 본다.

## 4. 저장

통계, 영작 문제, 사용자 답, 첨삭 결과를 `weekly/YYYY-Www.md`(ISO 주차, 예: `2026-W40.md`)에 저장한다.
같은 주 파일이 이미 있으면 아래에 이어서 추가한다.

마지막으로 커밋 명령을 제안하고, 승인하면 실행한다.

```
git add . && git commit -m "Add weekly writing practice for YYYY-Www"
```
