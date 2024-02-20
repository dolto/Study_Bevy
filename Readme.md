# Bevy엔진 예제 탐구 (2d기즈모)

## 목차
1. 소개
2. 과정
3. 결론

### 소개

- Bevy엔진은 Rust로 만들어진 ECS기반의 게임 엔진이다. (https://bevyengine.org/)
아직 초창기라 국내에서는 아는 사람을 찾는 것 조차 별로 없으나, 나름(?) 탄탄한 매니아층을 가지고 있어, 해외에서는 적잖은 인기를 얻고있다.

- 또한 ECS기반의 게임엔진은 Bevy가 독보적이라고 볼 수 있다 (유니티나 언리얼도 이와 같은 형태의 모드를 지원하는 것으로 알고 있지만, ECS안에서의 개발 편의성은 gui를 제외한 나머지는 Bevy가 가장 좋다고 볼 수 있다.)

- 이 프로젝트는 그냥 필자가 재밌어서 별로 없는 단계별 공부를 하다가, 그냥 아예 예제를 하나씩 분석하다보면 어느새 Bevy를 잘 다뤄서 게임을 만들 수 있는 수준까지 갈 수 있지 않을까 생각해서 진행하는 장기 프로젝트이다. 
- 이번 프로젝트 주제는 2d기즈모 예제코드를 분석하는 것이다.
- ~~스크린샷은 공부한 1시간 가량 정도의 시간을 녹화한 부분을 일부 캡쳐한 것이다.~~

### 과정

- 먼저 [bevy](https://github.com/bevyengine/bevy)를 ``` git clone ```으로 가져온 후, ``` cargo run --example ```을 입력하면 볼 수 있는 예제 이름 목록이 뜬다. (필자는 가장 위에서부터 순서대로 할 생각이다.)
![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_gizmos/1.webp)
- 이름을 알았으니 일단 실행해서 어떤 예제인지 알아보자.
![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_gizmos/2.webp)
- 다음과 같이 말 그대로 2d환경에서의 시각적 디버깅을 위한 기즈모를 그리는 예제를 볼 수 있다.
![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_gizmos/3.webp)
- 분석 과정은 다음과 같다. 코드를 위에서부터 천천히 읽으며, 모르는 기능이 있다면 공식 doc에서 해당 기능을 검색해서 어떤 기능인지 찾는 과정을 반복하는 것이다. (사진은 time.elapsed_seconds() 함수를 검색해 본 것이다.)
![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_gizmos/4.webp)
- 그렇게 대강 코드 분석이 끝나면, 코드를 약간 변형하는 형태로 처음부터 반절정도를 따라서 쓰며, 이해한 기능들을 다른형태로 활용해 보는 과정을 거친다.
- 필자의 경우 키보드를 누르면 기즈모의 선 두께가 변하는 것이 아닌, 움직이도록 코드를 변형했다.
![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_gizmos/5.webp)
- update_config함수는 원래 두께를 수정하는 역할도 했으나, local매개변수로 시스템에 지역변수를 넣었기 때문에 기즈모 그룹을 보이게만들거나 안보이게 만드는 토글만 구현했다.
![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_gizmos/6.webp)
- 완성된 변형 예제의 사진은 다음과 같다. 이제 여기서 방향키로 선과 네모 및 원을 움직일 수 있다.
![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_gizmos/7.webp)
- 실제 실행 화면은 다음과 같다
- ![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_gizmos/8.gif)

### 결론
- 요즘들어 게임 개발자가 되고 싶다는 사람이 자꾸 웹 쪽으로 가는 것 같아서 이런거라도 해야한다고 생각이 들었기에 시작한 프로젝트였다.
- 개발하는 과정을 실제로 녹화하며, 쉬는 시간 중간중간 일시정지를 눌렀더라도, 한시간 정도 공부하면 예제 하나정도는 읽을 수 있다는 결론이 나왔기에, 지속적으로 공부해볼 생각이다.
- 코드를 읽는 것만으로 끝내지 않아야 한다는 생각이 강하게 들었다. 왜냐하면 update_config함수는 마지막에 추가했는데, 그 전까지는 기즈모 그룹이 어떻게, 그리고 왜 쓰이는지 잘 이해가 가지 않았다가, 이를 직접 코드로 작성하다보니 놓친 부분들을 너무 많이 발견하게 되었기 때문이다.

[깃허브 링크](https://github.com/dolto/bevy_study_2d_gizmos)
