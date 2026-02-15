defmodule CedarPolicyTest do
  use ExUnit.Case
  doctest CedarPolicy

  test "greets the world" do
    ctx0 = CedarPolicy.new()

    assert CedarPolicy.increment(ctx0) == 1
    assert CedarPolicy.increment(ctx0) == 2
    assert CedarPolicy.increment(ctx0) == 3

    ctx1 = CedarPolicy.new()

    assert CedarPolicy.increment(ctx1) == 1
    assert CedarPolicy.increment(ctx0) == 4
  end
end
