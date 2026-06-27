class Test {
    int x;
    void main() {
        int a = x.y;
        int b = x.y.z;
        int c = Test.x.y;
    }
}