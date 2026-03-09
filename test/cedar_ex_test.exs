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

    policy0 = """
    permit(
      principal == User::"bob",
      action == Action::"view",
      resource == Album::"trip"
    ) when { principal.age >= 18 && context.boolean };
    """

    template0 = """
    permit(
      principal == ?principal,
      action == Action::"view",
      resource == ?resource
    ) when { principal.age >= 18 && context.boolean };
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

    p0 = EntityUid.new(EntityTypeName.new("User"), "bob")
    p1 = EntityUid.new(EntityTypeName.new("User"), "alice")
    a = EntityUid.new(EntityTypeName.new("Action"), "view")
    r = EntityUid.new(EntityTypeName.new("Album"), "trip")

    pb = Entity.new(p0, [{"age", {:long, 18}}])
    pa = Entity.new(p1, [{"age", {:long, 18}}])
    ab = Entity.new(a)
    rb = Entity.new(r)

    state =
      Native.new()
      |> Native.add_policy(policy0, "policy0")
      |> Native.add_template(template0, "template0")
      |> Native.add_entities([pb, pa, ab, rb], schema)
      |> Native.link("template0", "policy1", %{principal: p1, resource: r})

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

    dbg(Native.verify(state, p0, a, r, c, schema))
    dbg(Native.verify(state, p1, a, r, c, schema))
  end
end
