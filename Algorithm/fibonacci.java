import java.io.*;
import java.util.Scanner;

class main {
    public static void main(String[] args) {
        run_algo();
    }

    public static void run_algo() {
        int ix;
        int current_val = 1, previous_val = 0;

        System.out.print("Please enter Fibonacci index: ");
        ix = new Scanner(System.in).nextInt();

        for(int counter = 0; counter < ix; counter++) { 
            current_val += previous_val;
            if (counter != 0)
                previous_val = current_val - previous_val;
            System.out.print("-> " + current_val + " ");
        }
        System.out.println("");
    }
}