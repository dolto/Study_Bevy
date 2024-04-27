# Bevy엔진 예제 탐구 (Alien Cake Addict)
## 목차
1. 소개
2. 과정
3. 결론

### 소개
- 이번엔 간단한 게임으로, 제한시간 내에 케이크를 먹어야하는, 게임이다.
- bevy로 만든 게임의 로직이 어떤식으로 동작하는지 얼추 볼 수 있는 예제였다.

### 과정
[타임슬립](https://youtu.be/eBEq11AvkcM)
- ```cargo run --example alien_cake_addict```
- 수행하면 다음 게임을 플레이 해볼 수 있다. [링크](https://bevyengine.org/examples/Games/alien-cake-addict/)

- 한칸씩 외계인이 이동하면서 케이크를 먹으면 점수가 쌓이고, 케이크를 일정시간 내에 먹는 것을 실패하면 점수가 까이며, 일정 이상 점수가 까이면 게임오버하는 시스템이다.
- 로직 자체는 어렵지 않았으나, 이를 구현하는 코드들을 보는 도중 다음과 같은 형식의 랜덤을 구현하는 코드가 따로 존재했다.
- ![자료](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Bevy_Study_Alien_Cake/1.webp)
- bevy 내장 로직은 아닌 것 같고, 다른 외부 라이브러리에서 착안하여 따로 만든 기능인 것 같았으나, 필자는 대표적으로 사용하는 rust 라이브러리인 rand를 대신 활용했다.
- 또한 예제에서 타일은 그저 위치와 높이값만 있었는데, 울퉁불퉁한 타일을 보면서 이게 구불구불 움직이면 좋을 거 같아, 그렇게 수정하면 좋을 것 같아서 일부 수정을 가했다.
- 다음과 같이, Cell을 수정하고
- ![자료](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Bevy_Study_Alien_Cake/2.webp)
- 위아래로 이동하는 로직도 구현해준다
- ![자료](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Bevy_Study_Alien_Cake/3.webp)
- 그에 맞게 player와 cake의 위치를 조정하는 시스템을 추가해준다.
- ![자료](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Bevy_Study_Alien_Cake/4.webp)
- ![자료](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Bevy_Study_Alien_Cake/5.webp)

### 결론
- 코드 전체만 보면 양이 많아보이지만, 적절한 모듈화를 거치면 그렇게 많은 것도 아니고, ui를 제외하고 많은 부분을 살펴볼 수 있는 예제여서 흥미롭게 잘 보았다.
- [블로그](https://portfolio-user-8i5iwa8zn-doltos-projects.vercel.app/?is_blog=true&langs_slecets=[]&skills_slects=[]&project_id=662cde5ca5ed24162799e0df)

##### 자그마한 후기
- 요즘 취업 + 스터디 + rust 생태에서 front 웹을 다루는 기술 등을 살펴보면서, 현생이 바빠서 업로드를 잘 못했는데, 이번에 살피는 예제가 게임로직 코드여서 흥미롭게 잘 본 것 같다. (시간은 좀 걸렸지만)
