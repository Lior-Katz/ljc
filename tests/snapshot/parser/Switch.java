class SwitchExpressions {
    void switchStatements(Integer x) {
        switch (x) {
            case 5:
                x = 5;
                break;

            case null:
            default:
        }
    }

    void patterns(Object x) {
        switch (x) {
            case Integer i:
                i.intValue();
                break;

            case Pair(int a, int b):
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