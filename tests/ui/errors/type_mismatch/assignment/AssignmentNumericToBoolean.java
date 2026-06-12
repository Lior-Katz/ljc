class Test {
    void literal() {
        boolean bi; bi = 1;
        boolean bl; bl = 1L;
        boolean bc; bc = 'a';
    }

    void variable() {
        int i; i = 1;
        boolean bi; bi = i;

        long l; l = 1L;
        boolean bl; bl = l;

        char c; c = 'a';
        boolean bc; bc = c;
    }
}