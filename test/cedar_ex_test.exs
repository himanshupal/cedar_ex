defmodule CedarPolicyTest do
  use ExUnit.Case
  doctest CedarPolicy

  alias CedarPolicy.Entity
  alias CedarPolicy.EntityTypeName
  alias CedarPolicy.EntityUid
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

    templateJSON_0 =
      ~s({"effect":"permit","principal":{"op":"==","slot":"?principal"},"action":{"op":"==","entity":{"type":"Action","id":"view"}},"resource":{"op":"==","slot":"?resource"},"conditions":[{"kind":"when","body":{"&&":{"left":{">=":{"left":{".":{"left":{"Var":"principal"},"attr":"age"}},"right":{"Value":18}}},"right":{".":{"left":{"Var":"context"},"attr":"boolean"}}}}}]})

    templateJSON_1 =
      JSON.encode!(%{
        "effect" => "permit",
        "principal" => %{"op" => "==", "slot" => "?principal"},
        "action" => %{"op" => "==", "entity" => %{"type" => "Action", "id" => "view"}},
        "resource" => %{"op" => "==", "slot" => "?resource"},
        "conditions" => [
          %{
            "kind" => "when",
            "body" => %{
              "&&" => %{
                "left" => %{"<=" => %{"left" => %{"." => %{"left" => %{"Var" => "principal"}, "attr" => "age"}}, "right" => %{"Value" => 18}}},
                "right" => %{"." => %{"left" => %{"Var" => "context"}, "attr" => "boolean"}}
              }
            }
          }
        ]
      })

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

    v1 =
      CedarPolicy.new()
      |> CedarPolicy.add_policy(policy0, "policy0")
      |> CedarPolicy.add_entities([pb, pa, ab, rb], schema)
      |> CedarPolicy.add_template(template0, "template0")
      |> CedarPolicy.add_template({:json, templateJSON_0}, "templateJSON_0")
      |> CedarPolicy.add_template({:json, templateJSON_1}, "templateJSON_1")
      |> CedarPolicy.link("template0", "policy1", %{principal: p1, resource: r})

    v1
    |> CedarPolicy.validate_schema(schema, true)

    ns =
      ~s({"":{"commonTypes":{"ContextType":{"type":"Record","attributes":{"boolean":{"type":"Bool","required":true}}}},"entityTypes":{"User":{"shape":{"type":"Record","attributes":{"age":{"type":"Long"}}}},"Album":{"shape":{"type":"Record","attributes":{}}}},"actions":{"view":{"appliesTo":{"principalTypes":["User"],"resourceTypes":["Album"],"context":{"type":"ContextType"}}}}}})

    ne =
      ~s([{"uid":{"type":"User","id":"stacey"},"attrs":{"age":18},"parents":[]}])

    np =
      ~s({"effect":"permit","principal":{"op":"==","entity":{"type":"User","id":"bob"}},"action":{"op":"==","entity":{"type":"Action","id":"view"}},"resource":{"op":"==","entity":{"type":"Album","id":"trip"}},"conditions":[{"kind":"when","body":{"&&":{"left":{">=":{"left":{".":{"left":{"Var":"principal"},"attr":"age"}},"right":{"Value":18}}},"right":{".":{"left":{"Var":"context"},"attr":"boolean"}}}}}]})

    CedarPolicy.new()
    |> CedarPolicy.add_policy({:json, np}, "policyJ")
    |> CedarPolicy.add_entities({:json, ne}, {:json, ns})
    |> CedarPolicy.validate_schema({:json, ns})

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

    r1 = CedarPolicy.is_authorized(v1, p0, a, r, c, schema)
    r2 = CedarPolicy.is_authorized(v1, p1, a, r, c, schema)

    assert r1.authorized == false
    assert r2.authorized == true
  end
end
