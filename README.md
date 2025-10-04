# Discrete Mathematics: Inverse Matrix Calculator

이산수학 과제 - 역행렬 계산기

## 기능 (Features)

### 1. 행렬 입력 (Matrix Input)
- 사용자로부터 n×n 정방행렬을 행 단위로 입력
- 입력 검증 및 오류 처리

### 2. 두 가지 역행렬 계산 방법 (Two Inverse Calculation Methods)

#### 방법 1: 행렬식 기반 (Determinant Method)
- 여인수 행렬(Cofactor Matrix) 계산
- 수반 행렬(Adjugate Matrix) 계산
- 공식: A^-1 = (1/det(A)) × adj(A)

#### 방법 2: 가우스-조던 소거법 (Gauss-Jordan Elimination)
- 확대 행렬 [A | I] 생성
- 행 연산을 통한 소거

### 3. 결과 비교 및 검증 (Comparison & Verification)
- 두 방법의 결과 비교
- **자동 검증**: A × A^-1 = I 확인
- 수치 오차 분석

### 4. 분수 표시 모드 (Fraction Display Mode)
- `--fraction` 또는 `-f` 플래그 사용
- 정확한 유리수 표현 (1/3, 2/5 등)


## Rust 설치
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 사용법 (Usage)
### 기본 모드 
```bash
cargo run --release
```

### 계산 과정 출력 모드 
```bash
cargo run 
```

### 분수 모드
```bash
cargo run --release -- --fraction
# OR
cargo run -- --fraction
# OR
cargo run --release -- -f
# OR
cargo run -- -f
```

