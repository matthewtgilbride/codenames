from typing import Optional

import pytest
import requests


class TestState:
    game_name: Optional[str] = None
    current_turn: Optional[str] = None
    host: str = "http://localhost:8080"


@pytest.fixture(scope="session")
def host() -> str:
    return "http://localhost:8080"


@pytest.fixture(scope="session")
def test_state() -> TestState:
    return TestState()


def test_get_game_name(host, test_state):
    r = requests.get(f"{host}")
    assert r.status_code == 200
    game_name = r.json().get("game_name")
    assert game_name is not None
    test_state.game_name = game_name


def test_create_game(host, test_state):
    r = requests.post(f"{host}/game", json=vars(test_state))
    assert r.status_code == 200
    turns = r.json().get("turns")
    assert len(turns) == 1
    current_turn = turns[0].get("data")
    assert current_turn is not None
    test_state.current_turn = current_turn


def test_get_game(host, test_state):
    r = requests.get(f"{host}/game/{test_state.game_name}")
    assert r.status_code == 200
    assert r.json().get("type") == "State"
    cards = r.json().get("board")
    assert len(cards) == 25
    for card in cards:
        assert card.get("color") is None


def test_get_game_not_found(host):
    r = requests.get(f"{host}/game/foobar")
    assert r.status_code == 404


def test_get_all_game_names(host):
    r = requests.get(f"{host}/game")
    assert r.status_code == 200
    assert len(r.json().get("games")) > 0


def test_join_game_as_operative(host, test_state):
    r = requests.put(
        f"{host}/game/{test_state.game_name}/join",
        json={
            "name": "Mr Operative",
            "team": test_state.current_turn,
        },
    )

    assert r.status_code == 200
    assert r.json().get("type") == "State"
    assert r.json().get("players").get("mr operative") is not None
    assert r.json().get("players").get("mr operative").get("spymaster_secret") is None


def test_join_game_as_spy_master(host, test_state):
    r = requests.put(
        f"{host}/game/{test_state.game_name}/join",
        json={
            "name": "Mr Spy Master",
            "team": test_state.current_turn,
            "spymaster_secret": "foo",
        },
    )

    assert r.status_code == 200
    assert r.json().get("type") == "Data"
    assert r.json().get("players").get("mr spy master") is not None
    assert r.json().get("players").get("mr spy master").get("spymaster_secret") == "foo"


def test_join_game_player_exists(host, test_state):
    r = requests.put(
        f"{host}/game/{test_state.game_name}/join",
        json={
            "name": "Mr Operative",
            "team": test_state.current_turn,
            "spymaster_secret": "foo",
        },
    )

    assert r.status_code == 400
    assert r.json().get("msg") is not None


def test_start_turn(host, test_state):
    r = requests.put(
        f"{host}/game/{test_state.game_name}/mr spy master/start-turn",
        json={
            "word": "foo",
            "amount": 1,
        },
    )

    assert r.status_code == 200
    current_turn = r.json().get("turns")[0]
    assert current_turn.get("type") == "Started"
    assert current_turn.get("data").get("guesses") == []
    assert current_turn.get("data").get("clue")[0] == "foo"


def test_guess(host, test_state):
    r = requests.put(f"{host}/game/{test_state.game_name}/mr operative/guess/0")

    assert r.status_code == 200
    current_turn = r.json().get("turns")[0]
    assert current_turn.get("data").get("guesses")[0][0].get("name") == "Mr Operative"
    assert r.json().get("board")[0].get("color") is not None


def test_get_game_as_operative(host, test_state):
    r = requests.get(f"{host}/game/{test_state.game_name}/mr operative")

    assert r.status_code == 200
    assert r.json().get("board")[0].get("color") is not None
    assert r.json().get("board")[1].get("color") is None


def test_get_game_as_spy_master(host, test_state):
    r = requests.get(f"{host}/game/{test_state.game_name}/mr spy master")

    assert r.status_code == 200
    assert r.json().get("board")[0].get("color") is not None
    assert r.json().get("board")[1].get("color") is not None


def test_end_turn(host, test_state):
    r = requests.put(f"{host}/game/{test_state.game_name}/end-turn")

    assert r.status_code == 200
    turns = r.json().get("turns")
    assert len(turns) == 2
    assert turns[0].get("type") == "Pending"


def test_leave_game(host, test_state):
    r = requests.put(f"{host}/game/{test_state.game_name}/mr operative/leave")

    assert r.status_code == 200

    assert r.json().get("players").get("mr operative") is None
    assert r.json().get("players").get("mr spy master") is not None
