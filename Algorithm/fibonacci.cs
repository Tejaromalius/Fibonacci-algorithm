using System;

class Program {
    static void RunAlgo() {
        Console.Write("Please enter Fibonacci index: ");
        int ix = int.Parse(Console.ReadLine());

        int currentVal = 1, previousVal = 0;

        for (int counter = 0; counter < ix; counter++) {
            currentVal += previousVal;
            if (counter != 0) {
                previousVal = currentVal - previousVal;
            }
            Console.Write($"-> {currentVal} ");
        }
        Console.WriteLine();
    }

    static void Main() {
        RunAlgo();
    }
}

