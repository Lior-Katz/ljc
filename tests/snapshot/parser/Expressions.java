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

        // Shift
        x = 16 << 2 << 1;
        x = 16 >> 2 >> 1;
        x = 16 >>> 2 >>> 1;
        x = 16 << 2 >> 1;

        // Relational
        x = 5 < 10;
        x = 5 > 10;
        x = 5 <= 10;
        x = 5 >= 10;

        // Equality
        x = 5 == 5;
        x = 5 != 5;


        // =======================
        // Adjacent precedence
        // =======================

        // Multiplicative vs Additive
        x = 2 + 3 * 4;
        x = 2 * 3 + 4;

        // Additive vs Shift
        x = 2 + 3 << 1;
        x = 2 << 3 + 1;

        // Shift vs Relational
        x = 8 << 1 < 20;
        x = 8 < 1 << 4;

        // Relational vs Equality
        x = 5 < 10 == true;


        // =======================
        // Multi-level chains
        // =======================

        x = 2 + 3 * 4 << 1;
        x = 8 << 2 + 1 * 3;


        // =======================
        // Unary interaction
        // =======================

        x = -5 * 3;
        x = 5 * -3;
        x = -5 + -3;
        x = 5 + -3 * 2;
    }
}