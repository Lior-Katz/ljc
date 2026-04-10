class IfStatements {

    void test(int a, int b) {
        if (a > 0) a = 1;

        if (a > 0) a = 1;
        else a = 2;

        if (a > 0) {
            a = 1;
        } else {
            a = 2;
        }

        if (a > 0) if (b > 0) a = b;


        // else should bind to the inner if
        if (a > 0) if (b > 0) a = b;
        else a = -b;


        if (a > 0) {
            if (b > 0) a = b;
        } else {
            a = -b;
        }

        if (a > 0) a = 1;
        else if (a < 0) a = -1;
        else a = 0;

        if (a > 0) {
            if (b > 0) {
                a = b;
            } else {
                a = -b;
            }
        }

        if (a > 0) if (b > 0) if (a > b) a = b;
        else b = a;
    }
}