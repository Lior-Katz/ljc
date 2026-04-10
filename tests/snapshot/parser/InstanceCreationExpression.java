class InstaceCreationExpression {
    void unqualified_class_instance_creation() {
        new String();
        new Integer(5);
        new String(new String("abc"));
        String s = new String("hi") + new String("there");
        new String().compareTo(new String());
    }
}