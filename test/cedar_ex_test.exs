defmodule CedarPolicyTest do
  use ExUnit.Case
  doctest CedarPolicy

  alias CedarPolicy.EntityTypeName
  alias CedarPolicy.EntityUid
  alias CedarPolicy.Entity
  alias CedarPolicy.Native

  test "Works!" do
    CedarPolicy.get_lang_version()
    CedarPolicy.get_sdk_version()

    policy = """
    permit(
      principal == User::"bob",
      action == Action::"view",
      resource == Album::"trip"
    ) when { principal.age > 18 && context.boolean };
    """

    schema = """
        entity User {
          "age": Long
        };
        entity Album;
        action view appliesTo {
            principal : User,
            resource : Album,
            context : {
              "boolean": Bool,
            }
        };
    """

    p = EntityUid.new(EntityTypeName.new("User"), "bob")
    a = EntityUid.new(EntityTypeName.new("Action"), "view")
    r = EntityUid.new(EntityTypeName.new("Album"), "trip")

    pe = Entity.new(p, [{"age", {:long, 19}}])
    ae = Entity.new(a)
    re = Entity.new(r)

    state =
      Native.new()
      |> Native.add_policy(policy, "one")
      |> Native.add_entities([pe, ae, re], schema)

    assert Native.validate(state, schema)

    c = [
      {"boolean", {:bool, true}}
      # {"long", {:long, 123_456}},
      # {"ip", {:ip, "127.0.0.1"}},
      # {"string", {:string, "text"}},
      # {"decimal", {:decimal, "1.23"}},
      # {"datetime", {:date_time, "2015-01-13T13:00:07.001Z"}},
      # {"duration", {:duration, "24h"}},
      # {"entityUid", {:entity_uid, p}},
      # {"set", {:set, [{:long, 123}, {:string, "text"}]}},
      # {"record", {:record, [{"llong", {:ip, "192.168.1.1"}}, {"sstring", {:string, "text"}}]}}
    ]

    dbg(CedarPolicy.create_request(state, p, a, r, c, schema))
  end
end
