class DoWhileStatements {
    void test(int a, int b) {
        do
            a--;
        while (a > 0);

        do {
            a--;
            b++;
        } while (a > 0);

        do
            do
                b--;
            while (b > 0);
        while (a > 0);

        do
            if (b > 0)
                b--;
            else
                a--;
        while (a > 0);

        if (a > 0)
            do
                b--;
            while (b > 0);

        do {
            a--;
        } while ((a = a - 1) > 0);

        do ;
        while (a > 0);
    }
}