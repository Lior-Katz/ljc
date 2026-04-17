class ForStatements {
    void basic_for(int a, int b) {
        for (int i = 0; i < 10; i++)
            a += i;

        for (int i = 0, j = 10; i < j; i++, j--) {
            if (a == 5) continue;
            a += i + j;
        }

        for (; ; )
            break;

        for (; a < 10; )
            a++;

        loop:
        for (int i = 0; ; i++)
            break loop;

        for (int i = 0; (a = a - 1) > 0; i++)
            b++;

        int i = 0;
        for (i = 0, a = 1; i < 10; i++)
            a += i;

        for (int x = 0; x < 3; x++)
            for (int y = 0; y < 3; y++)
                a += x * y;
    }
}