def run_algo():
    index = int(input("Please enter Fibonacci index: "))
    
    current_val = 1
    previous_val = 0

    for ix in range(index):
        current_val += previous_val
        if ix != 0:
            previous_val = current_val - previous_val
        print(f"-> {current_val} ", end="")

    print()

if __name__ == "__main__":
    run_algo()