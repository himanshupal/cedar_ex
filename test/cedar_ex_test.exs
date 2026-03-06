defmodule CedarPolicyTest do
  use ExUnit.Case
  doctest CedarPolicy

  test "greets the world" do
    dbg(CedarPolicy.get_lang_version())
    dbg(CedarPolicy.get_sdk_version())

    # Policy init

    p1 =
      CedarPolicy.new("""
      permit(
      principal == User::"bob",
      action == Action::"view",
      resource == Album::"trip"
      )
      when { principal.age > 18 };
      """)

    dbg(CedarPolicy.get_policy_as_json(p1))
  end
end
