class Test {
    void literal() {
        byte b; b = true;
        short s; s = true;
        int i; i = true;
        long l; l = true;
        char c; c = true;
        float f; f = true;
        double d; d = true;
    }

    void variable() {
        boolean b = true;
        byte x; x = b;
        short y; y = b;
        int z; z = b;
        long w; w = b;
        char c; c = b;
        float f; f = b;
        double d; d = b;
    }
}