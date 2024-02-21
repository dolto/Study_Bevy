
# Bevy엔진 예제 탐구 (2d 셰이프)

## 목차
1. 소개
2. 과정
3. 결론

### 소개
- bevy 엔진의 깃허브에는 2d shapes 예제가 있다.
- 말 그대로 bevy엔진에서 어떻게 2d 모양을 그리는지 보여주는 형태이다.
![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_shapes/0.webp)
- 실행화면
![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_shapes/1.webp)
- 단순히 여러모양을 다양한 색상으로 출력하는 형태며, sprite형태의 어떤 리소스가 없더라도 간단한 모양정도는 코드로만 구현이 가능하다는 것을 알 수 있는 예제다.
- 추가적으로 메시 혹은 컬러 머터리얼의 개념을 활용하고 있었다.
- 공부를 위해 가장 오른쪽의 원을 움직이게 하는 기능도 추가하는 형태로 해당 예제를 분석했다.

### 과정
- [공부 과정 녹화본 (15배속)](https://youtu.be/4twJOdgVS3s)
- 코드에 나오는 메시나 다른 컬러 머터리얼을 추가하는 코드를 보면서 찾아본 문서들 (지금보면 약간 잘못 찾았긴 한데, 어쩼든 개념에 대해서 이해하고자 본 것이기 때문에 괜찮다고 생각한다.)

- 아무튼 메시는 실제 그래픽스에 나오는 버텍스와 같은 실질적으로 그래픽카드가 렌더링을 어떻게 해야하는지 보여주는 정보를 가지고 있는듯 했다. (~~간단하게 폴리곤과 같은 형태? 물론 정확한 이야기는 아니다.~~)
![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_shapes/2.webp)
- 또한 그러한 자료들을 불러오고 조작하는 데에 모든 데이터를 가져오면 성능적으로 굉장히 좋지 않으므로, 이를 강력 혹은 약한 참조를 하는 핸들러를 이용한다.
![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_shapes/3.webp)
- 머터리얼메시2d번들은 bevy엔진에서 어떤 엔티티를 생성할 때 자주 생성되는 컴포넌트들을 모아둔 말 그대로 번들이다.
- 다음사진은 해당 번들의 내용물들을 보여준다.
![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_shapes/4.webp)
- 마지막으로 Assets< Mesh >와 같은 형태의 자료가 어떻게 생겼는지 확인하는 부분이다.
- 기본적으로 에셋은 핸들러로 조작하며, 이에대한 ID를 저장하여 필요할 때 해시맵으로 불러온다.
- 문서에도 적혀있듯 런타임 시에만 에셋을 분류할 수 있다고 하니, 아마 저장 불러오기할 때 해당 자산은 다른 형태로 저장해야 할 것이다.~~(아마도)~~
![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_shapes/4.5.webp)
- 추가적으로 조종가능한 엔티티를 넣기 위해서 임의로 만든 Playerble컴포넌트를 넣어주고
![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_shapes/5.webp)
- 이를 업데이트 하는 코드까지 작성하고, 분석을 마쳤다.
![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_shapes/6.webp)
- 실행화면
![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_2d_shapes/7.gif)
- 실행화면에서 그래픽이 겹칠 때 깜빡거리는 것은 아마 z-index가 같은 형태이기 때문에 그런 것 같다.

### 결론
- 이번 예제는 간단하였기 때문에 생각보다 빨리 끝났다고 생각했다...
- 그러나 정리 글을 쓰면서, 코드 안에 나온 개념의 키워드를 검색하고 이를 이해하려고 하였지만, 애초에 검색 키워드가 잘못 되었다는 것이 있었다는 것을 알고, 중간에 한번 더 검색하고 코드를 다시 읽어보았다. (Assets< Mesh >를 보고 Mesh를 키워드로 검색할 게 아니라 Assets을 먼저 봤어야 했다.)
- 대강 분석하고 넘어가는 느낌이 있으나, 예제 코드는 매우 많고, 궁금하다고 파다보면 끝도 없기 때문에, (어떤 자산에 대한 핸들에도 강한 참조와 약한 참조나 다른 뻗어나갈 정보들이 많았다.) 중간에 일단 멈춘 거기도 하다. (단순하게 예제코드에서 바라는 것은 활용법을 이해하기만 하면 되는 것 같았기 때문에 멈춘 거기도 하다.)

[블로그 링크]()https://portfolio-user-6e57lkist-doltos-projects.vercel.app/project?is_blog=true&langs_slecets=[]&skills_slects=[]&project_id=65d57614f787a0687c520a56
