import pytest
from fastapi.testclient import TestClient
from server import app

client = TestClient(app)


def test_echo_with_dict_params():
    response = client.post(
        "/rpc",
        json={
            "jsonrpc": "2.0",
            "method": "echo",
            "params": {"message": "hello"},
            "id": 1,
        },
    )
    assert response.status_code == 200
    assert response.json() == {"jsonrpc": "2.0", "result": "hello", "id": 1}


def test_echo_with_list_params():
    response = client.post(
        "/rpc", json={"jsonrpc": "2.0", "method": "echo", "params": ["hi"], "id": 2}
    )
    assert response.status_code == 200
    assert response.json() == {"jsonrpc": "2.0", "result": "hi", "id": 2}


def test_invalid_jsonrpc_version():
    response = client.post(
        "/rpc",
        json={
            "jsonrpc": "1.0",
            "method": "echo",
            "params": {"message": "fail"},
            "id": 3,
        },
    )
    assert response.status_code == 400
    assert response.json()["error"]["code"] == -32600


def test_method_not_found():
    response = client.post(
        "/rpc", json={"jsonrpc": "2.0", "method": "not_exist", "params": {}, "id": 4}
    )
    assert response.status_code == 200
    assert response.json()["error"]["code"] == -32601
