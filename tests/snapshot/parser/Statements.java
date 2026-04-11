class Statements {
    void assignment_statements(int a, int b, int c, int d, int x) {
        // =======================
        // Basic assignments
        // =======================

        a = 1;
        a = b;
        a = b + c;


        // =======================
        // Right associativity
        // =======================

        // should parse as: a = (b = c)
        a = b = c;

        // deeper chaining
        a = b = c = d;


        // =======================
        // Assignment + arithmetic
        // =======================

        // RHS precedence
        a = b + c * d;
        a = (b + c) * d;

        // assignment inside expression
        a = (b = c) + d;


        // =======================
        // Assignment vs conditional
        // =======================

        // assignment has LOWER precedence than ?:

        // expect: a = (true ? c : d)
        x = a = true ? c : d;

        // expect: (a = b) ? c : d
        boolean bool;
        x = (bool = true) ? c : d;


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
        a *= (b + c);
        a <<= b + 1;


        // =======================
        // Chaining with compound
        // =======================

        // should parse as: a += (b += c)
        a += b += c;

        // mixed
        a = b += c = d;


        // =======================
        // Unary + assignment
        // =======================

        a = -b;
        a += -b;
        a = ++b;
        a = b++;
    }

    void prefix_operators(int a, int b) {
        ++a;
        --a;
        ++(a);
        --(b);
    }

    void postfix_operators(int a, int b) {
        a++;
        a--;
        (a)++;
        (b)--;
    }

    void block_statements() {
        {}
        {
            int a = 1;
        }
        {
            int a = 1;

            {
                {
                    int b = 2;
                }
            }

            int c = 3;
        }
    }

    void variable_declarations() {
        // =======================
        // Simple declarations
        // =======================

        int a1;
        boolean a2;
        String a3;


        // =======================
        // With initialization
        // =======================

        int b1 = 1;
        int b2 = b1 + 2;
        int b3 = (b1 + b2) * 2;
        int b4 = b1 > b2 ? b1 : b2;
        String b5 = "hello";


        // =======================
        // Multiple declarators
        // =======================

        int c1, c2, c3;
        int c4 = 1, c5, c6 = c5 = c4, c7;
        int c8, c9 = 5;


        // =======================
        // Initialization with assignment
        // =======================

        int d1, d2;
        int d3 = d2 = d1 = 1;


        // =======================
        // Unnamed variables
        // =======================
        int _ = 2, e1;
        int _ = e1 = 5;
        int _ = 1, _ = 2, e2 = 3;
    }

    void assert_statements(int a, int b) {
        assert a > 0;
        assert b > 0 : "b must be positive";
        assert (a + b) > 0;
        assert a != b : a + " should not equal " + b;
    }
}