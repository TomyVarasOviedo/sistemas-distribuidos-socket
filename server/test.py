import pytest
from main import evalute_text


@pytest.mark.parametrize(
    "entrada, esperado",
    [
        ("1+2+3+4+5+6+7+8+9+10", 55),
        ("1*2+3^4-5*6+7/8+9/10", 54.775),
        ("1*(2+3^4)-5*(6+7)/(8+9/10)", 75.69662921438314607),
        ("1*(2+3^4)-5*((6+7)/(8+9/10))", 75.69662921438314607),
        ("2-2+2*4*6*(56-5-1)+32", 2432),
        ("10 + 5 * 2 - 8 / 4", 18),
        ("-5 * -5", 25),
        ("10 - -5", 15),
        ("0.1 + 0.2", 0.3),
        ("0 * (500 / 2.5)^3", 0),
        ("(2^3 + 2^2) / 2", 6),
        ("0 / 0", "Error"),
    ],
)
def test_evalute_text(entrada, esperado):
    assert evalute_text(entrada) == pytest.approx(esperado)
