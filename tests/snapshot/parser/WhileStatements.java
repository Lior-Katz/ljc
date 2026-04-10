class WhileStatements {
    void test(int a, int b) {
        while (a > 0) a--;

        while (a > 0) {
            a--;
            b++;
        }

        while (a > 0)
            while (b > 0)
                b--;

        while (a > 0) {
            while (b > 0) {
                b--;
            }
            a--;
        }

        while (a > 0)
            if (b > 0) b--;
            else a--;

        if (a > 0) while (b > 0) b--;

        while (a > 0);
    }

    void danglingElse(int a, int b) {
        // else should bind to the INNER if
        if (a > 0)
            while (b > 0)
                if (a > b) a--;
                else b--;

        // else belongs to OUTER if
        if (a > 0) {
            while (b > 0)
                if (a > b) a--;
        } else {
            b--;
        }

        // else still binds to nearest if
        if (a > 0)
            if (b > 0)
                while (a > b) a--;
            else b--;
    }
}