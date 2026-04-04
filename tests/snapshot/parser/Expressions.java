public class Expressions {
    void literal_expressions() {
        x = 5;
        x = 10L;
        x = true;
        x = false;
        x = 'A';
        x = "abc";
        x = null;
    }

    void unary_operators() {
        x = x++;
        x = x--;
        x = ++x;
        x = --x;
        x = ~100;
        x = !!false;
        x = +3;
        x = -3;
    }

    void binary_operators() {
        // =======================
        // Associativity tests
        // =======================

        // Multiplicative
        x = 8 / 4 / 2;
        x = 8 * 4 * 2;
        x = 8 % 4 % 2;
        x = 8 * 4 / 2 % 3;

        // Additive
        x = 5 - 3 - 1;
        x = 5 + 3 + 1;
        x = 5 + 3 - 1;


        // =======================
        // Adjacent precedence
        // =======================

        // Multiplicative vs Additive
        x = 2 + 3 * 4;
        x = 2 * 3 + 4;


        // =======================
        // Unary interaction
        // =======================

        x = -5 * 3;
        x = 5 * -3;
        x = -5 + -3;
        x = 5 + -3 * 2;
    }
}