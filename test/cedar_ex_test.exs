defmodule CedarPolicyTest do
  use ExUnit.Case
  doctest CedarPolicy

  alias CedarPolicy.EntityTypeName
  alias CedarPolicy.EntityUid

  test "greets the world" do
    dbg(CedarPolicy.get_lang_version())
    dbg(CedarPolicy.get_sdk_version())

    p1 =
      CedarPolicy.new("""
      permit(
      principal == User::"bob",
      action == Action::"view",
      resource == Album::"trip"
      )
      when { principal.age > 18 };
      """)

    CedarPolicy.get_policy_as_json(p1)

    p = EntityUid.new(EntityTypeName.new("User", "Admin"), "123")
    a = EntityUid.new(EntityTypeName.new("View", "Access"), "456")
    r = EntityUid.new(EntityTypeName.new("Photo", "Travel"), "789")

    CedarPolicy.create_request(p, a, r)
  end
end
