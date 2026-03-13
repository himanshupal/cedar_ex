defmodule CedarPolicy do
  @moduledoc """
  Documentation for `CedarPolicy`.
  """

  alias CedarPolicy.Entity
  alias CedarPolicy.EntityUid
  alias CedarPolicy.Error
  alias CedarPolicy.Native
  alias CedarPolicy.Record
  alias CedarPolicy.TemplateParams

  @type entities :: list(Entity.t()) | {:list, list(Entity.t())} | {:json, String.t()}
  @type policy :: String.t() | {:cedar, String.t()} | {:json, String.t()}
  @type schema :: String.t() | {:cedar, String.t()} | {:json, String.t()}
  @type template :: String.t() | {:cedar, String.t()} | {:json, String.t()}

  @spec new :: reference()
  @spec get_sdk_version :: Version.t()
  @spec get_lang_version :: Version.t()
  @spec add_template(store :: reference(), template :: template(), id :: String.t() | nil) :: reference() | {:error, Error.t()}
  @spec add_policy(store :: reference(), policy :: policy(), id :: String.t() | nil) :: reference() | {:error, Error.t()}
  @spec add_entities(store :: reference(), entities :: entities(), schema :: schema() | nil) :: reference() | {:error, Error.t()}
  @spec link(store :: reference(), template_id :: String.t(), policy_id :: String.t(), values :: TemplateParams.t()) :: reference() | {:error, Error.t()}
  @spec validate(store :: reference(), schema :: schema(), strict :: boolean() | nil) :: reference() | {:error, Error.t()}
  @spec verify(store :: reference(), principal :: EntityUid.t(), action :: EntityUid.t(), resource :: EntityUid.t(), context :: Record.t() | nil, schema :: schema() | nil) ::
          boolean() | {:error, Error.t()}

  @doc """
  Get the Cedar SDK Semantic Versioning version
  """
  def get_sdk_version, do: Native.get_sdk_version()

  @doc """
  Get the Cedar language version
  """
  def get_lang_version, do: Native.get_lang_version()

  @doc """
  Creates an empty policy store
  """
  def new, do: Native.new()

  @doc """
  Adds a new policy in cedar policy language to the store
  """
  def add_policy(store, policy, id \\ nil)

  def add_policy(store, policy, id) when is_reference(store) and (is_nil(id) or is_binary(id)) do
    Native.add_policy(store, to_tuple(policy), id)
  end

  @doc """
  Adds a new template in cedar policy language to the store
  """
  def add_template(store, template, id \\ nil)

  def add_template(store, template, id) when is_reference(store) and (is_nil(id) or is_binary(id)) do
    Native.add_template(store, to_tuple(template), id)
  end

  @doc """
  Creates a new policy using provided template_id, policy_id & other template params
  """
  def link(store, template_id, policy_id, values) when is_reference(store) and is_binary(template_id) and is_binary(policy_id) and is_map(values) do
    Native.link(store, template_id, policy_id, values)
  end

  @doc """
  Add entities to the store
  """
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
  Validates the added policies against schema
  """
  def validate(store, schema, strict \\ false)

  def validate(store, schema, strict) when is_reference(store) and is_boolean(strict) do
    Native.validate(store, to_tuple(schema), strict)
  end

  @doc """
  Verify the access of principal for action on resource given context & optionally an schema
  """
  def verify(store, principal, action, resource, context \\ [], schema \\ nil)

  def verify(store, principal, action, resource, context, schema)
      when is_reference(store) and is_struct(principal, EntityUid) and is_struct(action, EntityUid) and is_struct(resource, EntityUid) and is_list(context) do
    Native.verify(store, principal, action, resource, context, to_tuple(schema))
  end

  defp to_tuple(data) when is_nil(data), do: nil
  defp to_tuple(data) when is_binary(data), do: {:cedar, data}
  defp to_tuple({type, value} = data) when is_tuple(data) and (type === :json or type === :cedar), do: {type, value}
end
