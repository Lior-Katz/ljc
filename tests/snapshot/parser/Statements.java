class Statements {
    void assignment_statements() {
        // =======================
        // Basic assignments
        // =======================

        a = 1;
        a = b;
        a = b + c;


        // =======================
        // Assignment + arithmetic
        // =======================

        // RHS precedence
        a = b + c * d;


        // =======================
        // Compound assignments
        // =======================

        a += b;
        a -= b;
        a *= b;
        a /= b;
        a %= b;

        a <<= b;
        a >>= b;
        a >>>= b;

        a &= b;
        a ^= b;
        a |= b;


        // =======================
        // Compound + expressions
        // =======================

        a += b + c * d;
        a <<= b + 1;


        // =======================
        // Unary + assignment
        // =======================

        a = -b;
        a += -b;
        a = ++b;
        a = b++;
    }
}