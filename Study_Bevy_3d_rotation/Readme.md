# Bevy엔진 예제 탐구 (3d 기즈모)

## 목차
1. 소개
2. 과정
3. 결론

### 소개
- 이번엔 게임개발에 가장 많이 쓰이는 Transform에서 rotation이 어떻게 동작하는지 보여주는 예제이다.

### 과정
- ~~이번엔 타임슬립 영상이 없습니다... 녹화본이 깨졌더라구요...~~
- 굉장히 간단한 예제이기 때문에, 빠르게 빠르게 작업했다... 심지어 딱히 추가적인 자료를 찾을정도의 어려움이 없었다!
- 왜 그런 지는 예제를 실행해 보면 된다.
![자료사진](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_rotation/1.5.webp)
![자료사진](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_rotation/2.webp)
- 보기만 해도 엄청 간단해 보인다...
- 다만 한 가지 헷갈리는 부분이 보인다. 바로 looking_at과 looking_to의 차이이다.
![자료사진](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_rotation/1.webp)
- 도큐먼트는 다음과 같이 말하고 있다.
![자료사진](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_rotation/3.webp)
- 읽어도 무슨말인지 잘 모르겠어서 다음과 같은 테스트 코드를 작성해 보았다.
![자료사진](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_rotation/3.5.webp)
- 실행 화면
- ![자료사진](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_rotation/4.webp)
- 이게 looking_to
- ![자료사진](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_rotation/5.webp)
- 이게 looking_at
- 얼핏 보면 looking_to는 아무것도 동작하지 않은 것 같지만, looking_to는 말 그대로 Vec::ZERO를 향해 바라보되, 각도를 조절하지 않는 다는 점에서 차이가 있었던 것이다.

- 최종 실행 화면
![자료사진](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_rotation/6.gif)

### 결론
- 사소한 거라도 다시보면 몰랐던 개념에 대해서 알게 될 수도 있으니, example은 그래왔든 계속 이럴 것이다. (아는거라고 넘어갔다가 나중에 후회할 일 만드는 것 보다 같은거 숙달하는게 더 좋아보인다.)

[블로그 링크](https://portfolio-user-o0cqsbyye-doltos-projects.vercel.app/project?is_blog=true&langs_slecets=[]&skills_slects=[]&project_id=65d73b95451edd16029acd7b)
