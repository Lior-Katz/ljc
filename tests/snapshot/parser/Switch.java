class SwitchExpressions {
    void switchStatements(Integer x) {
        switch (x) {
            case 0 -> x = 1;
            case 1, 2 -> {
                int y = 1;
                x = y;
            }
            case null, default -> throw new RuntimeException();
        }

        switch (x) {
            case 5:
                x = 5;
                break;

            case null:
            default:
        }
    }

    void switchExpressions(Integer x) {
        int a = switch (x) {
            case 0 -> 1;
            case 1, 2 -> {
                int y = 1;
                yield y;
            }
            case null, default -> throw new RuntimeException();
        };

        int b = switch (x) {
            case 0 -> 1;
            case null -> 3;
            default -> {
                yield 0;
            }
        };

        int c = switch (x) {
            case 5:
                yield 10;
            case 6:
            case 7:
                yield 20;
            default:
                yield 0;
        };
    }

    void patterns(Object x) {
        switch (x) {
            case Integer i:
                i.intValue();
                break;

            case Pair(int a, int b) when a > 5:
                int sum = a + b;
                break;

            case Box(Pair(int a, _)):
                int nested = a;
                break;

            case A(_), B(_):
            default:
                int a = 5;
        }
    }


    record Pair(int a, int b) {
    }

    record Box(Pair p) {
    }

    record A(int n) {
    }

    record B(boolean b) {
    }
}