import json;
import requests as r;

headers = {
    "content-type": "application/json"
};

def sled(params):
    data = {
        "id": "1",
        "method": "x",
        "jsonrpc": "2.0",
        "params": params
    };

    res = r.post('http://localhost:3030', data=json.dumps(data), headers=headers);
    print(res.text);

# insert author
class SledSets:
    def test_insert_kv():
        sled({
            'db': 'test_db',
            'key': 'test_key',
            'value': 'test_value'
        });
        
    def test_insert_kv_in_tree():
        sled({
            'db': 'test_db',
            'tree': 'test_tree',
            'key': 'test_key',
            'value': 'test_value'
        });

# st = SledSets;
# st.test_insert_kv();
# st.test_insert_kv_in_tree();
# st.test_insert_value();
class TestGets:
    def test_get_key():
        sled({
            'db': 'test_db',
            'key': 'test_key'
        });

    def test_get_key_in_tree():
        sled({
            'db': 'test_db',
            'tree': 'test_tree',
            'key': 'test_key'
        });

# tg = TestGets;
# tg.test_get_key();
# tg.test_get_key_in_tree();
class TestBatch:
    def test_batch_db():
        sled({
            'db': 'test_db',
            'batch': 'true'
        });

    def test_batch_db_in_tree():
        sled({
            'db': 'test_db',
            'tree': 'test_tree',
            'batch': 'true'
        });

tb = TestBatch;
tb.test_batch_db();
tb.test_batch_db_in_tree();
