class PermitsIdentifier {
    com.permits.Type permits;

    com.permits.Type permits(com.permits.Type permits) {
        return permits;
    }

    com.permits.Type method2() {
        int permits = 1;
        permits = permits + 1;
        com.permits.Type a;
        int c = Type.value + com.permits.Type.value;
        return new com.permits.Type();
    }

    enum per {
        permits
    }

    enum E2 {
        permits(1);

        E2(int permits) {
        }
    }

    enum E3 {
        permits {
            int permits = 1;
        };
    }

    enum E4 {
        permits(2) {
            int permits = 2;
        };

        E4(int permits) {
        }
    }
}