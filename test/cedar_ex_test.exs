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

    p = EntityUid.new(EntityTypeName.new("User"), "alice")
    a = EntityUid.new(EntityTypeName.new("Action"), "view")
    r = EntityUid.new(EntityTypeName.new("Album"), "trip")

    c = [
      {"long", {:long, 123_456}},
      {"boolean", {:bool, true}},
      {"ip", {:ip, "127.0.0.1"}},
      {"string", {:string, "text"}},
      {"decimal", {:decimal, "1.23"}},
      {"datetime", {:date_time, "2015-01-13T13:00:07.001Z"}},
      {"duration", {:duration, "24h"}},
      {"entityUid", {:entity_uid, p}},
      {"set", {:set, [{:long, 123}, {:string, "text"}]}},
      {"record", {:record, [{"llong", {:ip, "192.168.1.1"}}, {"sstring", {:string, "text"}}]}}
    ]

    s = """
        entity User;
        entity Album;
        action view appliesTo {
            principal : User,
            resource : Album,
            context : {
              "boolean": Bool,
              "string": String
            }
        };
    """

    CedarPolicy.create_request(p, a, r, c, s)
  end
end
