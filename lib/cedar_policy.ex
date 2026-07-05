defmodule CedarPolicy do
  @moduledoc """
  CedarPolicy is an Elixir wrapper for the [cedar-policy](https://crates.io/crates/cedar-policy) rust library using [rustler](https://github.com/rusterlium/rustler).

  There is not much change in how things work, except for a few changes that were required to make it work with Elixir.
  """

  alias CedarPolicy.AuthorizationResult
  alias CedarPolicy.Entity
  alias CedarPolicy.EntityUid
  alias CedarPolicy.Error
  alias CedarPolicy.Native
  alias CedarPolicy.Record
  alias CedarPolicy.SchemaValidationResult

  @typedoc """
  A type representing either a Cedar policy, schema or template in either Cedar format or JSON format.
  If the data is in JSON format then it must be provided as `{:json, policy}` where policy is a proper json string.

  If a string is provided then it is assumed to be in Cedar format.
  """
  @type json_or_cedar :: String.t() | {:cedar, String.t()} | {:json, String.t()}

  @typedoc """
  Representation of template parameters for creating a new policy using a pre-defined template.
  Either of the `principal` and `resource` must be provided & should be of type `CedarPolicy.EntityUid.t()`.
  """
  @type template_params :: %{principal: EntityUid.t() | nil, resource: EntityUid.t()} | %{principal: EntityUid.t(), resource: EntityUid.t() | nil}

  @doc """
  Get the Cedar SDK Semantic Versioning version.

  ## Example

      iex> CedarPolicy.get_sdk_version()
  """
  @spec get_sdk_version :: Version.t()

  def get_sdk_version, do: Native.get_sdk_version()

  @doc """
  Get the Cedar language version.

  ## Example

      iex> CedarPolicy.get_lang_version()
  """
  @spec get_lang_version :: Version.t()

  def get_lang_version, do: Native.get_lang_version()

  @doc """
  Creates an empty policy store.

  This is the very first function to be called as all the other functions depend on the store
  & expect it to be present for any policies, templates or entities to be added.

  ## Example

      iex> CedarPolicy.new()
  """
  @spec new :: reference()

  def new, do: Native.new()

  @doc """
  Adds a new policy to the store in either cedar policy language or json format.

  ## Example

  Adding policy in Cedar format

      iex> store = CedarPolicy.new()
      iex> policy = ~s(permit\\(principal == User::"bob", action == Action::"view", resource == Album::"trip"\\);)
      iex> CedarPolicy.add_policy(store, {:cedar, policy}, "policy0")
      iex> CedarPolicy.add_policy(store, policy, "policy1") # Or omit the `{:cedar, policy}` tuple and just provide the policy string

  Adding policy in JSON format

      # Adding policy in JSON format
      iex> store = CedarPolicy.new()
      iex> policy = ~s({"effect":"permit","principal":{"op":"==","entity":{"type":"User","id":"bob"}},"action":{"op":"==","entity":{"type":"Action","id":"view"}},"resource":{"op":"==","entity":{"type":"Album","id":"trip"}},"conditions":[]})
      iex> CedarPolicy.add_policy(store, {:json, policy}, "policy1")

  Returns the store reference if there is no error else a tuple of `{:error, CedarPolicy.Error}` is returned.
  """
  @spec add_policy(store :: reference(), policy :: json_or_cedar(), id :: String.t() | nil) :: reference() | {:error, Error.t()}

  def add_policy(store, policy, id \\ nil)

  def add_policy(store, policy, id) when is_reference(store) and (is_nil(id) or is_binary(id)) do
    Native.add_policy(store, to_tuple(policy), id)
  end

  @doc """
  Adds a new template to the store in either cedar policy language or json format.

  ## Example

  Adding template in Cedar format

      iex> store = CedarPolicy.new()
      iex> template = ~s(permit\\(principal == ?principal, action == Action::"view", resource == resource?"\\);)
      iex> CedarPolicy.add_template(store, {:cedar, template}, "template0")
      iex> CedarPolicy.add_template(store, template, "template1") # Or omit the `{:cedar, template}` tuple and just provide the template string

  Adding template in JSON format

      # Adding template in JSON format
      iex> store = CedarPolicy.new()
      iex> template = ~s({"effect":"permit","principal":{"op":"==","slot":"?principal"},"action":{"op":"==","entity":{"type":"Action","id":"view"}},"resource":{"op":"==","slot":"?resource"},"conditions":[]})
      iex> CedarPolicy.add_template(store, {:json, template}, "template1")

  Returns the store reference if there is no error else a tuple of `{:error, CedarPolicy.Error}` is returned.
  """
  @spec add_template(store :: reference(), template :: json_or_cedar(), id :: String.t() | nil) :: reference() | {:error, Error.t()}

  def add_template(store, template, id \\ nil)

  def add_template(store, template, id) when is_reference(store) and (is_nil(id) or is_binary(id)) do
    Native.add_template(store, to_tuple(template), id)
  end

  @doc """
  Creates a new policy using pre-defined template with its `template_id` & other template params.

  ## Example

      iex> store = CedarPolicy.new()
      iex> template = ~s(permit\\(principal == ?principal, action == Action::"view", resource == ?resource\\);)
      iex> CedarPolicy.add_template(store, template, "template0") # Add the template to the store
      iex> p1 = CedarPolicy.EntityUid.new(CedarPolicy.EntityTypeName.new("User"), "alice") # Create principal entity
      iex> r = CedarPolicy.EntityUid.new(CedarPolicy.EntityTypeName.new("Album"), "trip") # Create resource entity
      iex> CedarPolicy.link(store, "template0", "policy0", %{principal: p1, resource: r}) # The policy will be created with name `policy0` using the template `template0` with given principal & resource

  Returns the store reference if there is no error else a tuple of `{:error, CedarPolicy.Error}` is returned.
  """
  @spec link(
          store :: reference(),
          template_id :: String.t(),
          policy_id :: String.t(),
          values :: template_params()
        ) :: reference() | {:error, Error.t()}

  def link(store, template_id, policy_id, values) when is_reference(store) and is_binary(template_id) and is_binary(policy_id) and is_map(values) do
    Native.link(store, template_id, policy_id, values)
  end

  @doc """
  Add entities to the store & validate them against a schema if provided.

  ## Example

      iex> store = CedarPolicy.new()
      iex> principal_entity = CedarPolicy.Entity.new(CedarPolicy.EntityUid.new("User", "alice"))
      iex> action_entity = CedarPolicy.Entity.new(CedarPolicy.EntityUid.new("Action", "view"))
      iex> resource_entity = CedarPolicy.Entity.new(CedarPolicy.EntityUid.new("Album", "trip"))
      iex> CedarPolicy.add_entities(store, [principal_entity, action_entity, resource_entity])
  """
  @spec add_entities(store :: reference(), entities :: list(Entity.t()) | {:list, list(Entity.t())} | {:json, String.t()}, schema :: json_or_cedar() | nil) ::
          reference() | {:error, Error.t()}

  def add_entities(store, entities, schema \\ nil)

  def add_entities(store, entities, schema) when is_reference(store) and is_list(entities) do
    if Enum.all?(entities, &is_struct(&1, Entity)) do
      add_entities(store, {:list, entities}, schema)
    else
      raise FunctionClauseError
    end
  end

  def add_entities(store, {type, value} = entities, schema) when is_reference(store) and is_tuple(entities) and (type === :json or type === :list) do
    Native.add_entities(store, {type, value}, to_tuple(schema))
  end

  @doc """
  Validate all policies in a policy set, collecting all validation errors found into the returned result.

  ## Example

      iex> store = CedarPolicy.new()
      iex> policy = ~s(permit\\(principal == User::"bob", action == Action::"view", resource == Album::"trip"\\);)
      iex> schema = ~s(entity User {"age": Long}; entity Album; action view appliesTo {principal : User, resource : Album};)
      iex> CedarPolicy.add_policy(store, {:cedar, policy}, "policy0")
      iex> case CedarPolicy.validate_schema(store, schema, true) do
      ...>   %CedarPolicy.SchemaValidationResult{passed: true} -> nil # The validation passed, you can continue with the next steps
      ...>   {:error, error} -> IO.inspect(error) # Some kind of parsing error has occurred
      ...>   result -> result # The validation have failed
      ...> end
  """
  @spec validate_schema(store :: reference(), schema :: json_or_cedar(), strict :: boolean() | nil) :: {:error, Error.t()} | SchemaValidationResult.t()

  def validate_schema(store, schema, strict \\ false)

  def validate_schema(store, schema, strict) when is_reference(store) and is_boolean(strict) do
    Native.validate_schema(store, to_tuple(schema), strict)
  end

  @doc """
  Verify the access of principal for action on resource.

  ## Example

      iex> store = CedarPolicy.new()
      iex> policy = ~s(permit\\(principal == User::"bob", action == Action::"view", resource == Album::"trip"\\);)
      iex> CedarPolicy.add_policy(store, policy, "policy0")
      iex> principal_entity = CedarPolicy.Entity.new(CedarPolicy.EntityUid.new("User", "bob"))
      iex> action_entity = CedarPolicy.Entity.new(CedarPolicy.EntityUid.new("Action", "view"))
      iex> resource_entity = CedarPolicy.Entity.new(CedarPolicy.EntityUid.new("Album", "trip"))
      iex> CedarPolicy.add_entities(store, [principal_entity, action_entity, resource_entity])
      iex> case CedarPolicy.is_authorized(store, principal_entity.id, action_entity.id, resource_entity.id) do
      ...>  %CedarPolicy.AuthorizationResult{authorized: true} -> nil # The access is authorized, you can continue with the next steps
      ...>  {:error, error} -> IO.inspect(error) # Some kind of parsing error has occurred
      ...>  result -> result # The access is not authorized, check the `result` for more details
      ...> end
  """
  @spec is_authorized(
          store :: reference(),
          principal :: EntityUid.t(),
          action :: EntityUid.t(),
          resource :: EntityUid.t(),
          context :: Record.t() | nil,
          schema :: json_or_cedar() | nil
        ) :: AuthorizationResult.t() | {:error, Error.t()}

  def is_authorized(store, principal, action, resource, context \\ [], schema \\ nil)

  def is_authorized(store, principal, action, resource, context, schema)
      when is_reference(store) and is_struct(principal, EntityUid) and is_struct(action, EntityUid) and is_struct(resource, EntityUid) and is_list(context) do
    Native.is_authorized(store, principal, action, resource, context, to_tuple(schema))
  end

  defp to_tuple(data) when is_nil(data), do: nil
  defp to_tuple(data) when is_binary(data), do: {:cedar, data}
  defp to_tuple({type, value} = data) when is_tuple(data) and (type === :json or type === :cedar), do: {type, value}
end
