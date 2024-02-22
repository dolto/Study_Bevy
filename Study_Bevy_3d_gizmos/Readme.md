
# Bevy엔진 예제 탐구 (3d 기즈모)
## 목차
1. 소개
2. 과정
3. 결과

### 소개
- 이번엔 3d 기즈모 예제를 분석하고자 한다.
- 3d 기즈모에는 2d 기즈모와는 달리 기능들이 좀 더 많이 있으므로 그것을 중점으로 알아보도록 했다.
- 코드의 변형은 지난번과 같이 움직임을 추가하는 것으로 했으며, 이번엔 로컬 매개변수가 아닌, bevy에 리소스 구조체를 만들어서 활용했다.

### 과정
- [타임슬립](https://youtu.be/kkg7s4RVLLc)
- 먼저 ```cargo run --example 3d_gizmos``` 으로 예제를 실행시켜 본다.
![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_gizmos/1.webp)
- 예제에는 지난번 보다 더 많은 기능이 있었으니 차례대로 설명하면
- 기즈모가 다른 투영물체에 영향받지 않고 보이게 만드는 모드
- 기즈모에 원근감을 넣는 모드
- 각 기즈모 그룹의 두께 조절
- 각 기즈모 그룹의 보이기 설정
- 3d 메쉬의 정점에 기즈모 생성 기능이 있다.
---
- 코드를 하나하나 따라 쳐보면서, 이해가 안되거나 처음 보는 부분들을 검색하는 과정을 거쳤다.
- Pbr렌더링에 대해서 검색해보고 물리적 렌더 기법의 경우를 검색해보고, 표면에 따른 빛 반사에 대한 설정같은게 있어야 하는데, 이는 아마 머테리얼을 변경하거나 설정하여 이를 테스트할 수도 있을 것 같다.
![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_gizmos/2.webp)
- 또한 코드 상에서 모든 기즈모 그룹을 순회하면서 설정값을 변경하는 방법과
- depth_bias와 line_perspective 속성이 뭔지 알게 되었다.
![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_gizmos/3.webp)
- 실행결과
![자료화면](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_gizmos/4.gif)

### 결론
- 기즈모는 이번에 세번째라서 딱히 큰 실수는 없었으나, 리소스 등록에 약간의 문제가 있어서 조금 헤맨정도로 예제분석을 마칠 수 있었다.
- 논외지만, 사용중인 helix 에디터도 점점 손에 익어가는 느낌이라 좋았다 

[블로그 링크](https://portfolio-user-o0cqsbyye-doltos-projects.vercel.app/project?is_blog=true&langs_slecets=[]&skills_slects=[]&project_id=65d6d09e7504c636dcd64c7e)
