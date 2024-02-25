
# Bevy엔진 예제 탐구 (3d 셰이프)

## 목차
1. 소개
2. 과정
3. 결론

### 소개
- bevy엔진에서 3d 셰이프를 어떻게 생성하고, 머터리얼은 어떻게 적용하는지 알아보는 간단한 예제이다.
![자료사진](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_shapes/0.webp)

### 과정
[타임슬립](https://youtu.be/MaAHXuHwo5w)
- 먼저 example리스트를 보면서 시작한다.
- ![자료사진](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_shapes/1.webp)
- 3d_scene은 그 자체만으로 너무 간단해서 scene을 구현하는 예제와 병합해서 수행하려고 했으나, 예상치 못한 에러로 계속 자료를 찾는중이라 스킵했다.
- 예제 자체는 어려울 것이 없었으나, 자체적으로 모델들이 회전하는 것을 직접 조종하는 방식으로 변환하는데 약간의 어려움이 있었다.
- 쿼터니언을 오일러로 변환하는 부분에서 문제가 있었는데 이는 문서를 보고 해결했다.
- ![자료사진](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_shapes/2.webp)
- ![자료사진](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_shapes/3.webp)
- 다음으로 주목할 점은 텍스쳐를 직접 만든다는 부분이었다.
- ![자료사진](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_shapes/4.webp)
- Bevy에서 텍스쳐는 Image로 처리하며, 이에대한 코드는 과거 wgpu를 공부할 때 얼핏 본 느낌만 나기 때문에 간단하게 gpt한테 코드해석을 부탁했다.
- ![자료사진](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_shapes/5.webp)
- ![자료사진](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_shapes/6.webp)
- 어느정도 코드를 설명해줬으니 이제 문서를 보자.
- 대강 문서에선, image를 제공받은 픽셀 데이터로 한 면에 해당 픽셀 데이터로 렌더링하는 코드라고 되어있다.
- ![자료사진](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_shapes/7.webp)
- 그렇다면 다음 배열의 내장함수인 rotate_right(4)는 뭘까? 이를 주석처리하고 실행하면 다음과 같다.
- ![자료사진](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_shapes/10.webp)
- 보다시피 한 줄 씩 같은색상으로 변경된 것을 볼 수 있다. 말 그대로 배열의 순서를 마치 순환 큐 형태로 오른쪽으로 미는 것과 같은 효과를 낸다는 것을 알 수 있다.
- (다만 배열크기만큼의 복잡도를 제공하니 여러번 수행하는 거면 조치를 취해야 할 것이다.)
- ![자료사진](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_shapes/9.webp)
- 아무튼 그와같은 행동을 하는건 바로 8*8의 이미지를 갖고, 각각의 경계선을 명확하게 하기 위함이라는 것을 알 수 있었다.
- ![자료사진](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_shapes/11.webp)
![자료사진](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_shapes/12.webp) 
![자료사진](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_shapes/13.webp)
- 실행결과
- ![자료사진](https://raw.githubusercontent.com/dolto/port_folio_imgs/master/icon/projectsite/Study_Bevy_3d_shapes/14.gif)

### 결론
- 이번 탐구는 그래픽스와 관련된 부분이 많았기 때문에 찾아볼 정보들이 많았다.
- 이제 여기서 에셋을 이용해서 실제 사진이나 혹은 텍스쳐를 이용한 애니메이션 등은 어떻게 만드는지, 또 머터리얼을 이용한 깊이 효과는 어떻게 하는건지 궁금증이 많이 생기는 탐구였다.
