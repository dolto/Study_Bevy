
# Bevy엔진 예제 탐구 (3d 뷰포트를 월드좌표로)
## 목차
1. 소개
2. 과정
3. 결론

### 소개
- 지난번에 했던 2d 뷰포트 좌표를 카메라를 기반해서 월드좌표로 변환하는 예제와 일맥상 통하지만, 이번엔 ray3d를 사용 하였기 때문에 이점에 주목했다.
- 점진적으로 bevy의 기능에 점점 익숙해지기 위해서 오늘도 시작하도록 하겠다.

### 과정
[타임슬립](https://youtu.be/vstYCu19138)
- 과정은 언제나와 같이 예제 목록을 확인하여 위에서부터 차례대로 수행한다.
- 제목에도 나와있듯 이번엔 다음 코드를 입력하여 수행한다.
- ```cargo run --example 3d_viewport_to_world```
- 수행하면 다음과 같은 화면이 나오게 된다.
- ![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_viewport_to_world/2.webp)
- 마우스를 이동하면 해당 평면에 (평면이 없더라도) 기즈모가 그려지는 것을 알 수 있다.
- 아무튼 이번엔 마우스가 어떤 오브젝트에 닿았을 경우, 그 오브젝트의 색상이 변화하는 것을 목표로 시작했다.
- 코드가 익숙하지 않아서, 중간중간 헤매게 되는데, 일단 알게된 사실부터 나열하면
- ray는 오브젝트를 감지하지 않는다. 다만 오브젝트의 위치로부터 가상의 평면을 생성하여 그 평면에 닿는 형태로 오브젝트를 간접적으로 확인한다.
- 문서에도 ray로 활용하는 함수 또한 위치에서 가상의 평면과의 거리와, 그 거리가 닿은 위치를 반환하는 기능 뿐이었다.
- ![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_viewport_to_world/1.webp)
- 여기서 미래에 어려울 것 같은 원인은 바로 위치에서 어느 방향이 위인 평면을 만들 것인가이다... 만약 벽을 만든다면 그 벽은 어느 방향에서 볼 때에 따라서 ray에 막히는 평면 방향이 전부 다를 것인데, 이는 나중에 구현해봐야 할 것 같다.
---
- 두번째로, 어떤 엔티티에 관한 법선 기즈모는 출력은 불가능 한 것 같다. 본래 처음 시도했을 때는 AabbGizmoConfigGroup을 활용해서 어떤 엔티티에 관한 기즈모 그리기를 활성화 시킨다던지 하는 방법이 있을 줄 알았는데, 없었다

- ---
- 마지막으로 ray는 어떤 방향만 가질 뿐 어떤 오브젝트에 닿았다고 해서 없어지지 않는다.
- 아무튼 수행한 결과는 다음과 같다.
- 아직 bevy에서 콜라이더를 다룰 줄 몰라서 일단 큐브 크기만큼의 거리로 마우스가 가깝다면, 큐브가 밝은 노란색이 되도록 수행했다.
- 실행결과
- ![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_viewport_to_world/3.gif)
### 결론
- bevy에 익숙해지려면 아직 멀었다는 생각이 들었다.
- 이렇게 하나하나 활용해보면 예제는 차고 넘치고, 아는 지식이 많아질 수록 변환도 좀 더 과감하게 할 수 있기 때문에 아마 점진적으로 효과가 있지 않을까 싶다. ~~(아자아자!)~~

[블로그 링크](https://portfolio-user-kohl.vercel.app/?is_blog=true&langs_slecets=[]&skills_slects=[]&project_id=65dc9d29eb00f20fe76dca32)
