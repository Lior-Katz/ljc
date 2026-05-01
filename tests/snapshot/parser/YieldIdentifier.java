class RecordIdentifier {
    int yield;
    yield.Type field1;
    com.yield.Type field2;

    int yield(int yield) {
        return yield;
    }

    yield.Type method1(yield.Type yield) {
        return null;
    }

    com.yield.Type method2() {
        int yield = 1;
        yield = yield + 1;
        yield.Type a;
        com.yield.Type b;
        int c = yield.Type.value + com.yield.Type.value;
        return null;
        yield:
        while (true) {
            break yield;
        }
    }

    enum E1 {
        yield
    }

    enum E2 {
        yield(1);

        E2(int x) {
        }
    }

    enum E3 {
        yield {
            int x = 1;
        };
    }

    enum E4 {
        yield(2) {
            int y = 2;
        };

        E4(int x) {
        }
    }
}