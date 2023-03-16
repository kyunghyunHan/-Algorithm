# DataStructure

## Array

- 데이터의 논리적 순서와 물리적 순서가 동일
- 원칙적으로 데이터에 대한 접근 시간은 동일
- 데이터의 삽입과 삭제시 추가적인 자료의 이동이 발생

## Linked List

- 각 데이터 시퀀스가 순서를 가지고 연결된 순차적 구조
- 동적인 데이터 추가/삭제에 유리하다.
- 각 요소는 Node
- 각 Node에는 key와 다음 노드를 가리키는 포인터인 next가 포함
- 첫 번째 요소는 Head
- 마지막 요소는 Tail

## Stack

- 순서가 보존되는 선형 데이터 구조
- 가장 마지막 요소부터 처리하는 LIFO

## Que

- 가장 먼저 입력된 요소를 처리하는 FIFO
- 멀티 스레딩에서 스레드를 관리

## Heap

- Binary Tree
- 최소힙:부모의 키 값이 자식의 키 값보다 작거나 같다
- - 루트 노드의 키 값이 트리의 최솟값
- 최대힙:부모의 키 값이 자식의 키 값보다 크거나 같다.
- - 루트 노드의 키 값이 트리의 최솟값

## Hash table

- 해시함수를 사용하여 변환한 값을 색인으로 삼아 키와 데이터를 저장하는 자료구조

## Tree

- 그래프가 계층적 구조를 가진 형태
- 최상위 노드를 가지고 있다.
- 상위 노드를 parent node ,하위 노드를 child node라 한다.
- Binary Trees
- Binary Search Tree
- Heap

## Graph

- nodes/vertices사이에 edge가 있는 collection
- directed그래프는 일방통행
- undirected그래프는 양방향
- 소셜미디어 네트워크를 나타내는데사용
