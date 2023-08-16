# include <stdio.h>

void run_algo();

int main() {
    run_algo();
    return 0;
}

void run_algo() {
    int ix;
    int current_val = 1, previous_val = 0;

    printf("Please enter Fibonacci index: "); scanf("%d", &ix);

    for(int counter = 0; counter < ix; counter++) { 
        current_val += previous_val;
        if (counter != 0)
            previous_val = current_val - previous_val;
        printf("-> %d ", current_val);
    }
    puts("");
}