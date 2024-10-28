
# Bevy엔진 예제 탐구 (2d 뷰포트 좌표를 카메라 좌표로)

## 목차
- 소개
- 과정
- 결론

### 소개
- (아마) 모든 게임 엔진은 그래픽을 어떻게 렌더링 해야하는지 계산하는 개념이 있고, 그 개념의 중심에는 카메라라고 하는 눈 역할을 하는 viewr가 있다.
- 그런 viewr또한 위치를 가지고 있으니 렌더링이 보여지는 화면의 좌표와 차이가 있을 수 밖에 없으므로 그에대한 조정이 필요하다.
- 그런 뷰포트 좌표와 월드좌표를 카메라의 위치에 따라 동기화 시켜주는 예제를 분석해 보겠다.

### 과정
[타임슬립 영상 링크](https://www.youtube.com/watch?v=Yz_FR4HhdQo)
- 코드는 다음과 같다. [링크](https://bevyengine.org/examples/2D%20Rendering/2d-viewport-to-world/)
- 먼저 코드에 나오는 주요 함수를 살펴보자.
![자료](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_viewport_to_camera/0.webp)
- 즉 뷰포트에서 나오는 좌표를 카메라 위치에 기반해서 월드좌표로 변환해준다는 것이다.
- 해당 코드를 작성해준다.
![자료](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_viewport_to_camera/1.webp)
- 그렇게 해서 작성한 코드와 더불어, 예제에서 2d기즈모를 활용하길레, 이를 응용해보았다.
- 그러나 기즈모 그룹을 스스로 작성할 수 있는지 테스트 해 보았는데, 역시 한번 쓴 것만으로는 완벽하게 익히지 못한 것 같았다. (영상을 보면 15배속임에도 그 부분을 알아내기 위해 과거 예제를 수시로 살펴보는 것을 알 수 있을 것이다.)
![자료](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_viewport_to_camera/2.webp)
- 또한 위 코드가 안되고 아래 코드로 변환했어야 했는데, 가변 참조형은 원본에서 생성된 가변참조형의 활용 기간동안 다른 원본에서 생성된 가변참조형이 존재하면 안되기 때문이다.
- GPT선생님의 답변
![자료](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_viewport_to_camera/5.webp)
- .
![자료](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_viewport_to_camera/6.webp)
- 그렇게 해서 수정한 코드는 아래와 같다.
![자료](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_viewport_to_camera/3.webp)
- 실행하면 잘 실행되는 것을 알 수 있다.
![자료](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_viewport_to_camera/5.gif)

### 결론
- 이번 예제는 너무 간단하고, 변형하기에 아이디어가 쉽게 나오지 않았다.
- 그러나 예제의 주제를 표현하기 위해 이전에 배운 기즈모가 활용 되어서 그에 관련한 기능을 추가하는 부분에서 실수가 많이 나왔기 때문에, 그 실수를 바로 잡을 수 있는 기회가 되기도 했다.
- 앞으로도 이렇게 bevy 예제를 하나씩 분석할 생각이다.

[블로그 링크](https://portfolio-user-kohl.vercel.app/?is_blog=true&langs_slecets=[]&skills_slects=[]&project_id=65d5ccca8aa2756414842ac7)
