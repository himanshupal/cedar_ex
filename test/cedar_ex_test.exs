defmodule CedarPolicyTest do
  use ExUnit.Case
  doctest CedarPolicy

  alias CedarPolicy.EntityTypeName
  alias CedarPolicy.EntityUid
  alias CedarPolicy.Entity
  alias CedarPolicy.Native
  alias CedarPolicy.Record

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
        type UserId = {
          name: String
        };
        entity User {
          "age": Long
        };
        entity Album;
        action view appliesTo {
            principal : User,
            resource : Album,
            context : {
              "ip": ipaddr,
              "boolean": Bool,
              "atom": String,
              "string": String,
              "long": Long,
              "sett": Set<Long>,
              "user_set": Set<User>,
              "users": Set<UserId>,
              "entity": User,
              "decimal": decimal,
              "datetime": datetime,
              "duration": duration,
              "record": {
                "key1": String,
                "key2": decimal,
                "key3": {
                  "a": Long
                },
              }
            }
        };
    """

    p0 = EntityUid.new(EntityTypeName.new("User"), "bob")
    p1 = EntityUid.new(EntityTypeName.new("User"), "alice")
    a = EntityUid.new(EntityTypeName.new("Action"), "view")
    r = EntityUid.new(EntityTypeName.new("Album"), "trip")

    pb = Entity.new(p0, Record.new(age: 17))
    pa = Entity.new(p1, Record.new(age: 18))
    ab = Entity.new(a)
    rb = Entity.new(r)

    state =
      Native.new()
      |> Native.add_policy(policy0, "policy0")
      |> Native.add_template(template0, "template0")
      |> Native.add_entities([pb, pa, ab, rb], schema)
      |> Native.link("template0", "policy1", %{principal: p1, resource: r})
      |> Native.validate(schema)

    c =
      Record.new(
        long: 123,
        boolean: true,
        atom: :atom,
        string: "value",
        decimal: 123.456,
        entity: pb,
        sett: [1, 2, 3],
        user_set: [pb, pa],
        users: [Record.new(name: "bob"), Record.new(name: :alice)],
        datetime: ~U[2023-02-28T11:35:00.000Z],
        duration: "3s0ms",
        record: Record.new(key1: "value1", key2: 2.0, key3: Record.new(a: 1)),
        ip: "10.50.0.0/24"
      )

    dbg(Native.verify(state, p0, a, r, c, schema))
    dbg(Native.verify(state, p1, a, r, c, schema))
  end
end
