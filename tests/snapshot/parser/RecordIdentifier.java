class RecordIdentifier {
    int record;
    record.Type field1;
    com.record.Type field2;

    int record(int record) {
        return record;
    }

    record.Type method1(record.Type record) {
        return null;
    }

    com.record.Type method2() {
        int record = 1;
        record = record + 1;
        record.Type a;
        com.record.Type b;
        int c = record.Type.value + com.record.Type.value;
        return null;
    }
}