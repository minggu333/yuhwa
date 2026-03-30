def add(a, b):
    return a + b
def subtract(a, b):
    return a - b
def multiply(a, b):
    return a * b
def divide(a, b):
    if b == 0:
        return "오류: 0으로 나눌 수 없습니다"
    return a / b
def calculator():
    print("=" * 30)
    print("       간단한 계산기")
    print("=" * 30)
    while True:
        print("\n연산을 선택하세요:")
        print("1. 더하기 (+)")
        print("2. 빼기 (-)")
        print("3. 곱하기 (*)")
        print("4. 나누기 (/)")
        print("5. 종료")
        choice = input("\n선택 (1-5): ")
        if choice == "5":
            print("계산기를 종료합니다.")
            break
        if choice not in ["1", "2", "3", "4"]:
            print("잘못된 선택입니다. 1-5 중에서 선택하세요.")
            continue
        try:
            num1 = float(input("첫 번째 숫자: "))
            num2 = float(input("두 번째 숫자: "))
        except ValueError:
            print("오류: 올바른 숫자를 입력하세요.")
            continue
        if choice == "1":
            result = add(num1, num2)
            print(f"\n{num1} + {num2} = {result}")
        elif choice == "2":
            result = subtract(num1, num2)
            print(f"\n{num1} - {num2} = {result}")
        elif choice == "3":
            result = multiply(num1, num2)
            print(f"\n{num1} * {num2} = {result}")
        elif choice == "4":
            result = divide(num1, num2)
            print(f"\n{num1} / {num2} = {result}")
if __name__ == "__main__":
    calculator()
