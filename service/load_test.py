from typing import Optional
from os import environ
import uuid
from datetime import datetime
import time

import pytest
import requests


@pytest.fixture(scope="session")
def host() -> str:
    host_env = environ.get("TEST_URL")
    host = "http://localhost:8080" if host_env is None else host_env
    return host

@pytest.fixture(scope="session")
def game_name() -> str:
    return uuid.uuid4().hex


def test_create_game(host, game_name):
    r = requests.post(
        f"{host}/game",
        json={"game_name": game_name},
    )
    assert r.status_code == 200


def test_get_game(host, game_name):
    start = datetime.now()
    r = requests.get(f"{host}/game/{game_name}")
    status_code = r.status_code
    assert status_code == 200
    success_rate = 1
    while success_rate > 0.5:
        request_count = 0
        success_count = 0
        for _ in range(100):
            r = requests.get(f"{host}/game/{game_name}")
            request_count += 1
            if r.status_code == 200:
                success_count += 1
            time.sleep(1)
        success_rate = success_count / request_count
        print(f"{datetime.now().isoformat()}: {success_count}/{request_count}")
    end = datetime.now()
    assert start.isoformat() == end.isoformat()
